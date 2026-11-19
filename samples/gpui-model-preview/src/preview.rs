//! The GPUI widget: a resizable viewport that renders [`crate::scene`] on the
//! GPU, reads the frame back into a [`RenderImage`] and paints it through the
//! low-level canvas API.

use std::{cell::RefCell, path::{Path, PathBuf}, rc::Rc, sync::Arc, time::Instant};

use gpui_kit::component::ActiveTheme as _;
use gpui_kit::*;
use image::{Frame as ImageFrame, RgbaImage};

use crate::gpu::GpuRenderer;
use crate::model;
use crate::scene::{self, OrbitCamera, Scene};

/// User-facing knobs of the preview.
#[derive(Clone, Copy, Debug)]
pub struct PreviewOptions {
    pub auto_rotate: bool,
    pub show_ground: bool,
    /// Radians per second while auto-rotating.
    pub spin_speed: f32,
}

impl Default for PreviewOptions {
    fn default() -> Self {
        Self {
            auto_rotate: true,
            show_ground: true,
            spin_speed: 0.45,
        }
    }
}

/// Render statistics of the last painted frame.
#[derive(Clone, Debug, Default)]
pub struct PreviewStats {
    /// Smoothed render time, including the GPU readback.
    pub frame_ms: f32,
    /// Triangles submitted to the renderer in the last frame.
    pub triangles: usize,
    /// Size of the image that was uploaded, in physical pixels.
    pub pixels: (u32, u32),
    /// `GPU · <adapter>` or `CPU fallback`.
    pub backend: String,
}

struct PreviewState {
    camera: OrbitCamera,
    /// What "Reset view" restores; pulled back when the scene is a field of
    /// copies rather than a single model.
    home_camera: OrbitCamera,
    options: PreviewOptions,
    scene: Scene,
    scene_without_ground: Scene,
    /// What the header says is on screen.
    label: String,
    last_image: Option<Arc<RenderImage>>,
    last_tick: Option<Instant>,
    stats: PreviewStats,
    gpu: Option<GpuRenderer>,
    /// Set when wgpu could not be initialised; the CPU rasterizer is used then.
    gpu_error: Option<String>,
    dragging: bool,
    drag_position: Option<Point<Pixels>>,
}

/// A 3-D scene viewport: drag to orbit, scroll to zoom.
pub struct ScenePreview {
    state: Rc<RefCell<PreviewState>>,
}

impl ScenePreview {
    /// Builds the viewport. `model` is the FBX to show; `None`, or a file that
    /// fails to load, falls back to the demo torus knot. `copies` repeats the
    /// model across the stage without instancing, for stress tests.
    pub fn new(model: Option<PathBuf>, copies: usize) -> Self {
        let (scene, scene_without_ground, home_camera, label) =
            build_scenes(model.as_deref(), copies);
        Self {
            state: Rc::new(RefCell::new(PreviewState {
                camera: home_camera,
                home_camera,
                options: PreviewOptions::default(),
                scene,
                scene_without_ground,
                label,
                last_image: None,
                last_tick: None,
                stats: PreviewStats {
                    backend: "starting…".to_string(),
                    ..PreviewStats::default()
                },
                gpu: None,
                gpu_error: None,
                dragging: false,
                drag_position: None,
            })),
        }
    }

    /// One line describing what is on screen, for the header.
    pub fn label(&self) -> String {
        self.state.borrow().label.clone()
    }

    pub fn options(&self) -> PreviewOptions {
        self.state.borrow().options
    }

    pub fn stats(&self) -> PreviewStats {
        self.state.borrow().stats.clone()
    }

    pub fn set_auto_rotate(&mut self, value: bool, cx: &mut Context<Self>) {
        {
            let mut state = self.state.borrow_mut();
            if state.options.auto_rotate == value {
                return;
            }
            state.options.auto_rotate = value;
            state.last_tick = None;
        }
        cx.notify();
    }

    pub fn set_show_ground(&mut self, value: bool, cx: &mut Context<Self>) {
        {
            let mut state = self.state.borrow_mut();
            if state.options.show_ground == value {
                return;
            }
            state.options.show_ground = value;
        }
        cx.notify();
    }

    pub fn reset_camera(&mut self, cx: &mut Context<Self>) {
        {
            let mut state = self.state.borrow_mut();
            state.camera = state.home_camera;
            state.last_tick = None;
        }
        cx.notify();
    }

    fn viewport(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.clone();
        let view = cx.entity().downgrade();

        let prepaint_state = state.clone();
        let paint_state = state.clone();
        let canvas_view = view.clone();

        let viewport = canvas(
            move |bounds, window, _cx| {
                let (image, old) = render_frame(&prepaint_state, bounds, window);
                (image, old)
            },
            move |bounds, (image, old), window, _cx| {
                // The atlas keeps one tile per image id; the previous frame is
                // no longer referenced by the scene that is about to be drawn.
                if let Some(old) = old {
                    let _ = window.drop_image(old);
                }

                install_drag_handlers(&paint_state, &canvas_view, bounds, window);

                let radius = px(10.0);
                let _ = window.paint_image(bounds, bounds, Corners::all(radius), image, 0, false);
            },
        )
        .size_full();

        div()
            .id("scene-viewport")
            .flex_1()
            .min_h_0()
            .w_full()
            .relative()
            .overflow_hidden()
            .rounded_xl()
            .border_1()
            .border_color(cx.theme().border)
            .bg(rgb(0x05070d))
            .cursor(CursorStyle::OpenHand)
            .child(viewport)
            .on_scroll({
                let state = state.clone();
                let view = view.clone();
                move |event: &ScrollEvent, _window, cx| {
                    let delta = match event.delta {
                        ScrollDelta::Pixels(point) => point.y.as_f32(),
                        ScrollDelta::Lines(point) => point.y * 24.0,
                    };
                    if delta == 0.0 {
                        return;
                    }

                    state.borrow_mut().camera.zoom((delta * 0.0015).exp());
                    let _ = view.update(cx, |_, cx| cx.notify());
                }
            })
    }
}

impl Default for ScenePreview {
    fn default() -> Self {
        Self::new(None, 1)
    }
}

impl Render for ScenePreview {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        {
            let mut state = self.state.borrow_mut();
            if state.options.auto_rotate && !cx.reduce_motion() {
                let now = Instant::now();
                let delta = state
                    .last_tick
                    .map(|last| (now - last).as_secs_f32())
                    .unwrap_or(0.0)
                    .min(0.1);
                state.last_tick = Some(now);
                let spin_speed = state.options.spin_speed;
                state.camera.orbit(delta * spin_speed, 0.0);
                window.request_animation_frame();
            } else {
                state.last_tick = None;
            }
        }

        self.viewport(cx)
    }
}

/// Builds the two scene variants the widget toggles between — with the stage
/// and without it — plus the camera that frames them. A model that fails to
/// load is reported and replaced by the demo knot, so the viewport always has
/// something to draw.
fn build_scenes(
    model_path: Option<&Path>,
    copies: usize,
) -> (Scene, Scene, OrbitCamera, String) {
    let copies = copies.max(1);

    let (part, name, parts) = match model_path {
        None => (
            scene::demo_scene_parts(true, false, false),
            "torus knot".to_string(),
            None,
        ),
        Some(path) => match model::load(path) {
            Ok(loaded) => {
                log::info!(
                    "model {}: {} parts, {} materials, {} triangles (unit {:.3}, up axis {})",
                    path.display(),
                    loaded.parts,
                    loaded.materials,
                    loaded.triangles,
                    loaded.unit_scale,
                    loaded.up_axis,
                );

                let name = path
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| path.display().to_string());
                (loaded.scene, name, Some(loaded.parts))
            }
            Err(error) => {
                log::warn!(
                    "cannot load {}: {error}; showing the demo knot instead",
                    path.display()
                );
                (
                    scene::demo_scene_parts(true, false, false),
                    "torus knot".to_string(),
                    None,
                )
            }
        },
    };

    // Copies are plain triangles in the same soup: the renderer sees one big
    // scene and has no idea it is the same model repeated.
    let (part, span) = part.repeat(copies, scene::COPY_SPACING);
    let triangles = part.triangles.len();

    let mut with_ground = scene::demo_scene_parts(false, true, true);
    with_ground.widen_stage((span / scene::STAGE_SPAN).max(1.0));
    with_ground
        .triangles
        .extend(part.triangles.iter().copied());

    let mut label = name;
    if copies > 1 {
        label.push_str(&format!(" · ×{copies}"));
    }
    if let Some(parts) = parts {
        label.push_str(&format!(" · {parts} parts"));
    }
    label.push_str(&format!(" · {triangles} triangles"));

    // One copy keeps the demo framing; a field is pulled back so the whole
    // grid fits the viewport (the camera can zoom further out than it used to).
    let mut camera = OrbitCamera::default();
    camera.distance = camera.distance.max(span * 1.35);

    (with_ground, part, camera, label)
}

/// Renders the current state into a GPUI image at the widget's physical size.
fn render_frame(
    state: &Rc<RefCell<PreviewState>>,
    bounds: Bounds<Pixels>,
    window: &mut Window,
) -> (Arc<RenderImage>, Option<Arc<RenderImage>>) {
    let scale = window.scale_factor();

    // Render at physical pixel resolution, but cap the total pixel count so a
    // huge HiDPI window cannot make the readback the bottleneck.
    let mut width = (bounds.size.width.as_f32() * scale).round().max(1.0);
    let mut height = (bounds.size.height.as_f32() * scale).round().max(1.0);
    const MAX_PIXELS: f32 = 4_000_000.0;
    let budget = (MAX_PIXELS / (width * height)).sqrt().min(1.0);
    width = (width * budget).max(1.0);
    height = (height * budget).max(1.0);
    let width = width as u32;
    let height = height as u32;

    let mut state = state.borrow_mut();

    if state.gpu.is_none() && state.gpu_error.is_none() {
        match GpuRenderer::new() {
            Ok(renderer) => {
                log::info!("scene preview using GPU: {}", renderer.adapter_name());
                state.gpu = Some(renderer);
            }
            Err(error) => {
                log::warn!("GPU renderer unavailable, falling back to CPU: {error}");
                state.gpu_error = Some(error);
            }
        }
    }

    let PreviewState {
        camera,
        options,
        scene,
        scene_without_ground,
        gpu,
        stats,
        last_image,
        ..
    } = &mut *state;

    let scene = if options.show_ground {
        &*scene
    } else {
        &*scene_without_ground
    };

    let started = Instant::now();
    let frame = match gpu {
        Some(renderer) => {
            if stats.backend.is_empty() {
                stats.backend = format!("GPU · {}", renderer.adapter_name());
            }
            renderer.render(scene, camera, width, height)
        }
        None => {
            stats.backend = "CPU fallback".to_string();
            scene::render(scene, camera, width, height)
        }
    };
    let elapsed = started.elapsed();

    stats.frame_ms = smooth(stats.frame_ms, elapsed.as_secs_f32() * 1000.0);
    stats.triangles = frame.triangles;
    stats.pixels = (frame.width, frame.height);

    let buffer = RgbaImage::from_raw(frame.width, frame.height, frame.pixels)
        .expect("the renderer always returns width * height * 4 bytes");
    let image = Arc::new(RenderImage::new(vec![ImageFrame::new(buffer)]));
    let old = last_image.replace(image.clone());

    (image, old)
}

fn smooth(previous: f32, current: f32) -> f32 {
    if previous == 0.0 {
        current
    } else {
        previous * 0.88 + current * 0.12
    }
}

/// Registers window-level drag listeners. They are re-registered every frame
/// and only live for that frame, which is how GPUI's imperative input works.
fn install_drag_handlers(
    state: &Rc<RefCell<PreviewState>>,
    view: &WeakEntity<ScenePreview>,
    bounds: Bounds<Pixels>,
    window: &mut Window,
) {
    window.on_mouse_event({
        let state = state.clone();
        move |event: &MouseDownEvent, _phase, _window, _cx| {
            if event.button != MouseButton::Left || !bounds.contains(&event.position) {
                return;
            }
            let mut state = state.borrow_mut();
            state.dragging = true;
            state.drag_position = Some(event.position);
        }
    });

    window.on_mouse_event({
        let state = state.clone();
        let view = view.clone();
        move |event: &MouseMoveEvent, _phase, _window, cx| {
            let mut state = state.borrow_mut();
            if !state.dragging {
                return;
            }
            if !event.dragging() {
                state.dragging = false;
                state.drag_position = None;
                return;
            }

            let Some(previous) = state.drag_position.replace(event.position) else {
                return;
            };

            // ~0.6 degrees per logical pixel.
            let delta = event.position - previous;
            state.camera.orbit(-delta.x.as_f32() * 0.010, delta.y.as_f32() * 0.010);
            drop(state);

            let _ = view.update(cx, |_, cx| cx.notify());
        }
    });

    window.on_mouse_event({
        let state = state.clone();
        move |_event: &MouseUpEvent, _phase, _window, _cx| {
            let mut state = state.borrow_mut();
            state.dragging = false;
            state.drag_position = None;
        }
    });
}
