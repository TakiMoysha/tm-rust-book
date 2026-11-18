mod preview;
mod scene;

use gpui_kit::component::{
    ActiveTheme as _, StyledExt as _,
    button::{Button, ButtonVariants as _},
    h_flex,
    label::Label,
    switch::Switch,
    v_flex,
};
use gpui_kit::*;

use preview::ScenePreview;

struct DemoApp {
    preview: Entity<ScenePreview>,
}

impl DemoApp {
    fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            preview: cx.new(|_| ScenePreview::default()),
        })
    }
}

impl Render for DemoApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // let options = self.preview.read(cx).options();
        // let stats = self.preview.read(cx).stats();

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
                                Label::new("Torus knot rasterized on the CPU and painted as a GPUI image")
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground),
                            ),
                    )
                    .child(
                        h_flex().gap_4().items_center(), // .child(
                                                         // Switch::new("auto-rotate")
                                                         //     .checked(options.auto_rotate)
                                                         //     .label("Auto-rotate")
                                                         //     .on_change(move |checked, _, cx| {
                                                         //         auto_rotate.update(cx, |this, cx| {
                                                         //             this.set_auto_rotate(*checked, cx);
                                                         //         });
                                                         //     }),
                                                         // )
                                                         // .child(
                                                         //     Switch::new("show-ground")
                                                         //         .checked(options.show_ground)
                                                         //         .label("Ground")
                                                         //         .on_change(move |checked, _, cx| {
                                                         //             show_ground.update(cx, |this, cx| {
                                                         //                 this.set_show_ground(*checked, cx);
                                                         //             });
                                                         //         }),
                                                         // )
                                                         // .child(Button::new("reset-view").secondary().label("Reset view").on_click(
                                                         //     move |_, _, cx| {
                                                         //         reset.update(cx, |this, cx| this.reset_camera(cx));
                                                         //     },
                                                         // )),
                    ),
            )
            .child(self.preview.clone())
            .child(
                h_flex().w_full().justify_between().child(
                    Label::new("Drag to orbit · Scroll to zoom")
                        .text_sm()
                        .text_color(cx.theme().muted_foreground),
                ), // .child(
                   // Label::new(format!(
                   //     "{}×{} px · {} triangles · {:.1} ms",
                   //     stats.pixels.0, stats.pixels.1, stats.triangles, stats.frame_ms
                   // ))
                   //     .text_sm()
                   //     .text_color(cx.theme().muted_foreground),
                   // ),
            )
    }
}

fn main() {
    init_gpu_logging();

    // A headless escape hatch used to eyeball the rasterizer without opening a
    // window: `cargo run --release -- --snapshot out.png`.
    let mut args = std::env::args().skip(1);
    if let Some(flag) = args.next() {
        if flag == "--snapshot" {
            let path = args.next().unwrap_or_else(|| "preview.png".to_string());
            save_snapshot(&path);
            return;
        }
        if flag == "--bench" {
            bench();
            return;
        }
    }

    application().with_assets(assets::Assets).run(|cx| {
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
            |_, cx| DemoApp::new(cx),
        )
        .expect("failed to open window");
    });
}

fn bench() {
    let camera = scene::OrbitCamera::default();
    let cases = [
        ("empty", scene::Scene::default()),
        ("knot", scene::demo_scene_parts(true, false, false)),
        ("disc", scene::demo_scene_parts(false, true, false)),
        ("grid", scene::demo_scene_parts(false, false, true)),
        ("knot + ground", scene::demo_scene(true)),
    ];

    for (name, scene) in cases {
        for size in [(1000, 700)] {
            let started = std::time::Instant::now();
            let frame = scene::render(&scene, &camera, size.0, size.1);
            println!(
                "{name:>14} {:>4}x{:<4} {:>7.2} ms ({} tris)",
                size.0,
                size.1,
                started.elapsed().as_secs_f64() * 1000.0,
                frame.triangles
            );
        }
    }
}

fn save_snapshot(path: &str) {
    let scene = scene::demo_scene(true);
    let started = std::time::Instant::now();
    let frame = scene::render(&scene, &scene::OrbitCamera::default(), 1000, 700);
    let elapsed = started.elapsed();

    // The rasterizer emits BGRA for GPUI; image encoders expect RGBA.
    let mut rgba = frame.pixels;
    for pixel in rgba.chunks_exact_mut(4) {
        pixel.swap(0, 2);
    }

    image::save_buffer(path, &rgba, frame.width, frame.height, image::ExtendedColorType::Rgba8)
        .expect("failed to save snapshot");

    println!(
        "saved {} ({}×{}, {} triangles, {:.2} ms)",
        path,
        frame.width,
        frame.height,
        frame.triangles,
        elapsed.as_secs_f64() * 1000.0
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
