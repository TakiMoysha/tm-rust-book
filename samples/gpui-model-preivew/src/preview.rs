//! The GPUI widget: a resizable viewport that rasterizes [`crate::scene`] into
//! a [`RenderImage`] every frame and paints it through the low-level canvas API.

use std::{cell::RefCell, rc::Rc, sync::Arc, time::Instant};

use gpui_kit::component::ActiveTheme as _;
use gpui_kit::*;
use image::{Frame as ImageFrame, RgbaImage};

use crate::scene::{self, OrbitCamera, Scene};

// ===========================================================
// INTPU
// ===========================================================

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

// ======================================================================

actions!(counter, [Increment]);

pub struct ScenePreview {
    count: i32,
}

impl ScenePreview {
    fn increment(&mut self, _: &Increment, _: &mut Window, cx: &mut Context<Self>) {
        self.count += 1;
        cx.notify();
    }
}

impl Default for ScenePreview {
    fn default() -> Self {
        Self { count: 0 }
    }
}

impl Render for ScenePreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().key_context("Counter").on_action(cx.listener(Self::increment))
    }
}

fn bind_keys(cx: &mut App) {
    cx.bind_keys([KeyBinding::new("up", Increment, Some("Counter"))]);
}

// Render statistics of the last painted frame.
// #[derive(Clone, Copy, Debug, Default)]
// pub struct PreviewStats {
//     /// Smoothed CPU rasterization time.
//     pub frame_ms: f32,
//     /// Triangles submitted to the rasterizer in the last frame.
//     pub triangles: usize,
//     /// Size of the image that was uploaded, in physical pixels.
//     pub pixels: (u32, u32),
// }

// struct PreviewState {
//     camera: OrbitCamera,
//     options: PreviewOptions,
//     scene: Scene,
//     scene_without_ground: Scene,
//     last_image: Option<Arc<RenderImage>>,
//     last_tick: Option<Instant>,
//     // stats: PreviewStats,
//     dragging: bool,
//     drag_position: Option<Point<Pixels>>,
// }
//
// /// A 3-D scene viewport: drag to orbit, scroll to zoom.
// pub struct ScenePreview {
//     state: Rc<RefCell<PreviewState>>,
// }
//
// impl ScenePreview {
//     pub fn new() -> Self {
//         Self {
//             state: Rc::new(RefCell::new(PreviewState {
//                 camera: OrbitCamera::default(),
//                 options: PreviewOptions::default(),
//                 scene: scene::demo_scene(true),
//                 scene_without_ground: scene::demo_scene(false),
//                 last_image: None,
//                 last_tick: None,
//                 // stats: PreviewStats::default(),
//                 dragging: false,
//                 drag_position: None,
//             })),
//         }
//     }
//
//     pub fn options(&self) -> PreviewOptions {
//         self.state.borrow().options
//     }
//
//     // pub fn stats(&self) -> PreviewStats {
//     //     self.state.borrow().stats
//     // }
//
//     pub fn set_auto_rotate(&mut self, value: bool, cx: &mut Context<Self>) {
//         {
//             let mut state = self.state.borrow_mut();
//             if state.options.auto_rotate == value {
//                 return;
//             }
//             state.options.auto_rotate = value;
//             state.last_tick = None;
//         }
//         cx.notify();
//     }
//
//     pub fn set_show_ground(&mut self, value: bool, cx: &mut Context<Self>) {
//         {
//             let mut state = self.state.borrow_mut();
//             if state.options.show_ground == value {
//                 return;
//             }
//             state.options.show_ground = value;
//         }
//         cx.notify();
//     }
//
//     pub fn reset_camera(&mut self, cx: &mut Context<Self>) {
//         self.state.borrow_mut().camera.reset();
//         cx.notify();
//     }
//
//     fn viewport(&self, cx: &mut Context<Self>) -> impl IntoElement {
//         let state = self.state.clone();
//         let view = cx.entity().downgrade();
//
//         let prepaint_state = state.clone();
//         let paint_state = state.clone();
//         let canvas_view = view.clone();
//
//         let viewport = canvas(
//             move |bounds, window, _cx| {
//                 let (image, old) = render_frame(&prepaint_state, bounds, window);
//                 (image, old)
//             },
//             move |bounds, (image, old), window, _cx| {
//                 // The atlas keeps one tile per image id; the previous frame is
//                 // no longer referenced by the scene that is about to be drawn.
//                 if let Some(old) = old {
//                     let _ = window.drop_image(old);
//                 }
//
//                 install_drag_handlers(&paint_state, &canvas_view, bounds, window);
//
//                 let radius = px(10.0);
//                 let _ = window.paint_image(bounds, bounds, Corners::all(radius), image, 0, false);
//             },
//         )
//         .size_full();
//
//         div()
//             .id("scene-viewport")
//             .flex_1()
//             .min_h_0()
//             .w_full()
//             .relative()
//             .overflow_hidden()
//             .rounded_xl()
//             .border_1()
//             .border_color(cx.theme().border)
//             .bg(rgb(0x05070d))
//             .cursor(CursorStyle::OpenHand)
//             .child(viewport)
//             .on_scroll({
//                 let state = state.clone();
//                 let view = view.clone();
//                 move |event: &ScrollEvent, _window, cx| {
//                     let delta = match event.delta {
//                         ScrollDelta::Pixels(point) => point.y.as_f32(),
//                         ScrollDelta::Lines(point) => point.y * 24.0,
//                     };
//                     if delta == 0.0 {
//                         return;
//                     }
//
//                     state.borrow_mut().camera.zoom((delta * 0.0015).exp());
//                     let _ = view.update(cx, |_, cx| cx.notify());
//                 }
//             })
//     }
// }
//
// impl Default for ScenePreview {
//     fn default() -> Self {
//         Self::new()
//     }
// }
//
// impl Render for ScenePreview {
//     fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//         {
//             let mut state = self.state.borrow_mut();
//             if state.options.auto_rotate && !cx.reduce_motion() {
//                 let now = Instant::now();
//                 let delta = state
//                     .last_tick
//                     .map(|last| (now - last).as_secs_f32())
//                     .unwrap_or(0.0)
//                     .min(0.1);
//                 state.last_tick = Some(now);
//                 let spin_speed = state.options.spin_speed;
//                 state.camera.orbit(delta * spin_speed, 0.0);
//                 window.request_animation_frame();
//             } else {
//                 state.last_tick = None;
//             }
//         }
//
//         self.viewport(cx)
//     }
// }
//
// /// Rasterizes the current state into a GPUI image at the widget's physical size.
// fn render_frame(
//     state: &Rc<RefCell<PreviewState>>,
//     bounds: Bounds<Pixels>,
//     window: &mut Window,
// ) -> (Arc<RenderImage>, Option<Arc<RenderImage>>) {
//     let scale = window.scale_factor();
//
//     // Render at physical pixel resolution, but cap the total pixel count so a
//     // huge HiDPI window cannot make the CPU rasterizer the bottleneck.
//     let mut width = (bounds.size.width.as_f32() * scale).round().max(1.0);
//     let mut height = (bounds.size.height.as_f32() * scale).round().max(1.0);
//     const MAX_PIXELS: f32 = 1_000_000.0;
//     let budget = (MAX_PIXELS / (width * height)).sqrt().min(1.0);
//     width = (width * budget).max(1.0);
//     height = (height * budget).max(1.0);
//     let width = width as u32;
//     let height = height as u32;
//
//     let mut state = state.borrow_mut();
//     let scene = if state.options.show_ground {
//         &state.scene
//     } else {
//         &state.scene_without_ground
//     };
//
//     let started = Instant::now();
//     let frame = scene::render(scene, &state.camera, width, height);
//     let elapsed = started.elapsed();
//
//     // state.stats.frame_ms = smooth(state.stats.frame_ms, elapsed.as_secs_f32() * 1000.0);
//     // state.stats.triangles = frame.triangles;
//     // state.stats.pixels = (frame.width, frame.height);
//
//     let buffer = RgbaImage::from_raw(frame.width, frame.height, frame.pixels)
//         .expect("rasterizer always returns width * height * 4 bytes");
//     let image = Arc::new(RenderImage::new(vec![ImageFrame::new(buffer)]));
//     let old = state.last_image.replace(image.clone());
//
//     (image, old)
// }
//
// fn smooth(previous: f32, current: f32) -> f32 {
//     if previous == 0.0 {
//         current
//     } else {
//         previous * 0.88 + current * 0.12
//     }
// }
//
// /// Registers window-level drag listeners. They are re-registered every frame
// /// and only live for that frame, which is how GPUI's imperative input works.
// fn install_drag_handlers(
//     state: &Rc<RefCell<PreviewState>>,
//     view: &WeakEntity<ScenePreview>,
//     bounds: Bounds<Pixels>,
//     window: &mut Window,
// ) {
//     window.on_mouse_event({
//         let state = state.clone();
//         move |event: &MouseDownEvent, _phase, _window, _cx| {
//             if event.button != MouseButton::Left || !bounds.contains(&event.position) {
//                 return;
//             }
//             let mut state = state.borrow_mut();
//             state.dragging = true;
//             state.drag_position = Some(event.position);
//         }
//     });
//
//     window.on_mouse_event({
//         let state = state.clone();
//         let view = view.clone();
//         move |event: &MouseMoveEvent, _phase, _window, cx| {
//             let mut state = state.borrow_mut();
//             if !state.dragging {
//                 return;
//             }
//             if !event.dragging() {
//                 state.dragging = false;
//                 state.drag_position = None;
//                 return;
//             }
//
//             let Some(previous) = state.drag_position.replace(event.position) else {
//                 return;
//             };
//
//             // ~0.6 degrees per logical pixel.
//             let delta = event.position - previous;
//             state.camera.orbit(-delta.x.as_f32() * 0.010, delta.y.as_f32() * 0.010);
//             drop(state);
//
//             let _ = view.update(cx, |_, cx| cx.notify());
//         }
//     });
//
//     window.on_mouse_event({
//         let state = state.clone();
//         move |_event: &MouseUpEvent, _phase, _window, _cx| {
//             let mut state = state.borrow_mut();
//             state.dragging = false;
//             state.drag_position = None;
//         }
//     });
// }
