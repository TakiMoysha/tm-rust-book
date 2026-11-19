mod gpu;
mod model;
mod preview;
mod scene;

use std::path::{Path, PathBuf};

use gpui_kit::component::{
    ActiveTheme as _, StyledExt as _,
    button::{Button, ButtonVariants as _},
    h_flex,
    label::Label,
    switch::Switch,
    v_flex,
};
use gpui_kit::*;

use gpu::GpuRenderer;
use preview::ScenePreview;

struct DemoApp {
    preview: Entity<ScenePreview>,
}

impl DemoApp {
    fn new(model: Option<PathBuf>, copies: usize, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            preview: cx.new(|_| ScenePreview::new(model, copies)),
        })
    }
}

impl Render for DemoApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let options = self.preview.read(cx).options();
        let stats = self.preview.read(cx).stats();
        let label = self.preview.read(cx).label();

        let auto_rotate = self.preview.clone();
        let show_ground = self.preview.clone();
        let reset = self.preview.clone();

        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .p_5()
            .gap_4()
            .child(
                h_flex()
                    .w_full()
                    .items_end()
                    .justify_between()
                    .child(
                        v_flex()
                            .gap_1()
                            .child(div().text_2xl().font_semibold().child("3D scene preview"))
                            .child(
                                Label::new(format!("{label} · rendered with wgpu as a GPUI image"))
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_4()
                            .items_center()
                            .child(
                                Switch::new("auto-rotate")
                                    .checked(options.auto_rotate)
                                    .label("Auto-rotate")
                                    .on_change(move |checked, _, cx| {
                                        auto_rotate.update(cx, |this, cx| {
                                            this.set_auto_rotate(*checked, cx);
                                        });
                                    }),
                            )
                            .child(
                                Switch::new("show-ground")
                                    .checked(options.show_ground)
                                    .label("Ground")
                                    .on_change(move |checked, _, cx| {
                                        show_ground.update(cx, |this, cx| {
                                            this.set_show_ground(*checked, cx);
                                        });
                                    }),
                            )
                            .child(Button::new("reset-view").secondary().label("Reset view").on_click(
                                move |_, _, cx| {
                                    reset.update(cx, |this, cx| this.reset_camera(cx));
                                },
                            )),
                    ),
            )
            .child(self.preview.clone())
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .child(
                        Label::new("Drag to orbit · Scroll to zoom")
                            .text_sm()
                            .text_color(cx.theme().muted_foreground),
                    )
                    .child(
                        h_flex()
                            .gap_3()
                            .child(
                                Label::new(stats.backend.clone())
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground),
                            )
                            .child(
                                Label::new(format!(
                                    "{}×{} px · {} triangles · {:.1} ms",
                                    stats.pixels.0, stats.pixels.1, stats.triangles, stats.frame_ms
                                ))
                                .text_sm()
                                .text_color(cx.theme().muted_foreground),
                            ),
                    ),
            )
    }
}

/// The model shown when `--model` is not given.
const DEFAULT_MODEL: &str = "assets/example.fbx";

enum Action {
    Snapshot(String),
    Bench,
}

fn main() {
    init_gpu_logging();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut model = Some(PathBuf::from(DEFAULT_MODEL));
    let mut copies = 1;
    let mut action = None;

    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--model" => {
                index += 1;
                match args.get(index) {
                    Some(path) => model = Some(PathBuf::from(path)),
                    None => {
                        eprintln!("--model needs a path");
                        return;
                    }
                }
            }
            "--no-model" => model = None,
            "--copies" => {
                index += 1;
                match args.get(index).and_then(|value| value.parse::<usize>().ok()) {
                    Some(value) => copies = value.max(1),
                    None => {
                        eprintln!("--copies needs a positive number");
                        return;
                    }
                }
            }
            "--snapshot" => {
                let path = match args.get(index + 1) {
                    Some(next) if !next.starts_with("--") => {
                        index += 1;
                        next.clone()
                    }
                    _ => "preview.png".to_string(),
                };
                action = Some(Action::Snapshot(path));
            }
            "--bench" => action = Some(Action::Bench),
            other => eprintln!(
                "unknown argument {other:?} (known: --model <path>, --no-model, --copies <n>, --snapshot [png], --bench)"
            ),
        }
        index += 1;
    }

    match action {
        Some(Action::Snapshot(path)) => {
            save_snapshot(&path, model.as_deref(), copies);
            return;
        }
        Some(Action::Bench) => {
            bench(model.as_deref(), copies);
            return;
        }
        None => {}
    }

    application().with_assets(assets::Assets).run(move |cx| {
        init(cx);
        gpui_kit::component::Theme::change(gpui_kit::component::ThemeMode::Dark, None, cx);

        open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(120.0), px(120.0)),
                    size: size(px(1120.0), px(780.0)),
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("GPUI 3D scene preview".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            cx,
            |_, cx| DemoApp::new(model.clone(), copies, cx),
        )
        .expect("failed to open window");
    });
}

/// The model on the stage: the loaded mesh plus the demo ground and grid, or
/// the demo knot when there is nothing to load. `copies` repeats it across the
/// stage as plain triangles — no instancing, so the cost scales with the count.
fn stage_scene(model: Option<&Path>, copies: usize) -> (scene::Scene, f32) {
    let copies = copies.max(1);
    let part = match model {
        None => scene::demo_scene_parts(true, false, false),
        Some(path) => match model::load(path) {
            Ok(loaded) => {
                println!(
                    "model {}: {} parts, {} materials, {} triangles (unit {:.3}, up axis {})",
                    path.display(),
                    loaded.parts,
                    loaded.materials,
                    loaded.triangles,
                    loaded.unit_scale,
                    loaded.up_axis
                );
                loaded.scene
            }
            Err(error) => {
                eprintln!("model {}: {error}; showing the demo knot", path.display());
                scene::demo_scene_parts(true, false, false)
            }
        },
    };

    let (part, span) = part.repeat(copies, scene::COPY_SPACING);
    let mut stage = scene::demo_scene_parts(false, true, true);
    stage.widen_stage((span / scene::STAGE_SPAN).max(1.0));
    stage.triangles.extend(part.triangles);

    // The span lets the caller pull the camera back far enough to see it all.
    (stage, span)
}

fn bench(model: Option<&Path>, copies: usize) {
    let mut renderer = match GpuRenderer::new() {
        Ok(renderer) => renderer,
        Err(error) => {
            eprintln!("GPU renderer unavailable: {error}");
            return;
        }
    };
    println!("renderer: {}", renderer.adapter_name());

    let camera = scene::OrbitCamera::default();
    let mut cases = vec![
        ("empty".to_string(), scene::Scene::default()),
        ("knot".to_string(), scene::demo_scene_parts(true, false, false)),
        ("disc".to_string(), scene::demo_scene_parts(false, true, false)),
        ("grid".to_string(), scene::demo_scene_parts(false, false, true)),
        ("knot + ground".to_string(), scene::demo_scene(true)),
    ];
    let name = if copies > 1 {
        format!("model ×{copies} + ground")
    } else {
        "model + ground".to_string()
    };
    cases.push((name, stage_scene(model, copies).0));

    for (name, scene) in cases {
        for size in [(500, 350), (1000, 700), (1600, 1120)] {
            // The first draw compiles/uploads pipeline state; measure steady
            // state instead, including the readback that the widget also pays.
            let _ = renderer.render(&scene, &camera, size.0, size.1);

            let mut best = f64::INFINITY;
            let mut triangles = 0;
            for _ in 0..5 {
                let started = std::time::Instant::now();
                let frame = renderer.render(&scene, &camera, size.0, size.1);
                best = best.min(started.elapsed().as_secs_f64() * 1000.0);
                triangles = frame.triangles;
            }

            println!(
                "{name:>14} {:>4}x{:<4} {:>7.2} ms ({} tris)",
                size.0, size.1, best, triangles
            );
        }
    }
}

fn save_snapshot(path: &str, model: Option<&Path>, copies: usize) {
    let mut renderer = match GpuRenderer::new() {
        Ok(renderer) => renderer,
        Err(error) => {
            eprintln!("GPU renderer unavailable: {error}");
            return;
        }
    };

    let (scene, span) = stage_scene(model, copies);
    let mut camera = scene::OrbitCamera::default();
    camera.distance = camera.distance.max(span * 1.35);

    let started = std::time::Instant::now();
    let frame = renderer.render(&scene, &camera, 1000, 700);
    let elapsed = started.elapsed();

    // The renderer emits BGRA for GPUI; image encoders expect RGBA.
    let mut rgba = frame.pixels;
    for pixel in rgba.chunks_exact_mut(4) {
        pixel.swap(0, 2);
    }

    image::save_buffer(
        path,
        &rgba,
        frame.width,
        frame.height,
        image::ExtendedColorType::Rgba8,
    )
    .expect("failed to save snapshot");

    println!(
        "saved {} ({}×{}, {} triangles, {:.2} ms) via {}",
        path,
        frame.width,
        frame.height,
        frame.triangles,
        elapsed.as_secs_f64() * 1000.0,
        renderer.adapter_name()
    );
}

/// GPUI reports adapter selection through the `log` crate. A logger is only
/// needed to debug "which GPU did the window end up on"; without one the
/// records are dropped. Run with `RUST_LOG=info` semantics always on.
fn init_gpu_logging() {
    struct StderrLogger;

    impl log::Log for StderrLogger {
        fn enabled(&self, metadata: &log::Metadata) -> bool {
            metadata.level() <= log::Level::Info
        }

        fn log(&self, record: &log::Record) {
            if !self.enabled(record.metadata()) {
                return;
            }
            let target = record.target();
            if target.starts_with("gpui") || target.starts_with("wgpu") {
                eprintln!("[{}] {}: {}", record.level(), target, record.args());
            }
        }

        fn flush(&self) {}
    }

    static LOGGER: StderrLogger = StderrLogger;
    let _ = log::set_logger(&LOGGER);
    log::set_max_level(log::LevelFilter::Info);
}
