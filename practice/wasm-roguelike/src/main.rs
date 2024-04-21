use quicksilver::run;
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics},
    Input, Result, Settings, Window,
};

const DEFAULT_WINDOW_SIZE: Vector = Vector { x: 800.0, y: 600.0 };

pub mod lib {
    use std::{thread::sleep, time::Duration};

    use quicksilver::{
        geom::{Circle, Rectangle, Vector},
        graphics::{
            blend::{BlendChannel, BlendFactor, BlendFunction, BlendInput, BlendMode},
            Color, Element, FontRenderer, Graphics, Image, Mesh, PixelFormat, ResizeHandler,
            Surface, VectorFont, Vertex,
        },
        input::{Event, Key},
        Input, Result, Settings, Timer, Window,
    };

    fn draw_atlas(gfx: &mut Graphics, image: &Image) {
        // gfx.set_blend_mode(Some(BlendMode {
        //     equation: Default::default(),
        //     function: BlendFunction::Same {
        //         source: BlendFactor::Color {
        //             input: BlendInput::Source,
        //             channel: BlendChannel::Alpha,
        //             is_inverse: false,
        //         },
        //         destination: BlendFactor::Color {
        //             input: BlendInput::Source,
        //             channel: BlendChannel::Alpha,
        //             is_inverse: true,
        //         },
        //     },
        //     global_color: [0.0; 4],
        // }));

        let overlay_region = Rectangle::new(Vector::new(0.0, 0.0), image.size());
        gfx.draw_image(&image, overlay_region);
        gfx.fill_rect(&overlay_region, Color::BLUE.with_alpha(0.3));

        // gfx.set_blend_mode(Some(BlendMode {
        //     equation: Default::default(),
        //     function: BlendFunction::Same {
        //         source: BlendFactor::One,
        //         destination: BlendFactor::Zero,
        //     },
        //     global_color: [0.0; 4],
        // }));
    }

    pub async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
        let STEPS = 2;

        for i in 0..STEPS {
            draw_loader(&window, &mut gfx, i, STEPS);
            sleep(Duration::from_secs(1));
        }

        let asset_atlas = Image::load(&gfx, "assets/atlas.png").await?;
        let asset_font = VectorFont::load("assets/VividSans.ttf").await?;

        // draw atlas
        // draw_atlas(&mut gfx, &asset_atlas);

        // draw text
        let font_size = 42.0;
        let mut font = asset_font.to_renderer(&gfx, font_size)?;
        let font_color = Color::from_hex("#FAAA33");
        font.draw(
            &mut gfx,
            "Hello world\nQuicksilver!",
            font_color,
            Vector::new(32.0, 32.0),
        )?;

        // resize
        let vertices = {
            let top = Vertex {
                pos: Vector::new(400.0, 200.0),
                uv: None,
                color: Color::RED.with_alpha(0.2),
            };
            let left = Vertex {
                pos: Vector::new(200.0, 400.0),
                uv: None,
                color: Color::GREEN.with_alpha(0.2),
            };
            let right = Vertex {
                pos: Vector::new(600.0, 400.0),
                uv: None,
                color: Color::BLUE.with_alpha(0.2),
            };
            vec![top, left, right]
        };
        let elements = vec![Element::Triangle([0, 1, 2])];
        let mesh = Mesh {
            vertices,
            elements,
            image: Some(asset_atlas),
        };
        let resize_handler = ResizeHandler::Fit {
            aspect_width: 4.0,
            aspect_height: 3.0,
        };
        gfx.set_resize_handler(resize_handler);

        // timer
        // let mut update_timer = Timer::time_per_second(30.0);
        // let mut draw_timer = Timer::time_per_second(60.0);
        // let mut rect = Rectangle::new(Vector::new(0.0, 80.0), Vector::new(16.0, 16.0));

        // events
        let mut square_position = Vector::new(300.0, 300.0);
        let mut my_string = String::new();
        let mut is_running = true;

        while is_running {
            gfx.clear(Color::WHITE.with_alpha(0.8));

            while let Some(event) = input.next_event().await {
                match event {
                    Event::KeyboardInput(input_event) => {
                        if input_event.key() == Key::Escape {
                            is_running = false;
                        } else if input_event.key() == Key::Back && input_event.is_down() {
                            my_string.pop();
                        }
                    }
                    Event::ReceivedCharacter(c) => {
                        let chr = c.character();
                        if !chr.is_control() {
                            my_string.push(chr);
                        }
                    }
                    _ => {}
                }
            }

            // if input.key_down(Key::H) {}
            // if input.key_down(Key::J) {}
            // if input.key_down(Key::K) {}
            // if input.key_down(Key::L) {}

            // gfx.clear(Color::WHITE);
            font.draw_wrapping(
                &mut gfx,
                &my_string,
                Some(1200.0),
                Color::BLACK,
                Vector::new(100.0, 100.0),
            )?;
            gfx.draw_mesh(&mesh);
            gfx.present(&window)?;
        }

        Ok(())
    }

    fn draw_loader(
        window: &Window,
        gfx: &mut Graphics,
        progress: usize,
        total: usize,
    ) -> Result<()> {
        gfx.clear(Color::BLACK);
        gfx.fill_rect(
            &Rectangle::new(Vector::new(50.0, 500.0), Vector::new(700.0, 25.0)),
            Color::YELLOW,
        );

        let width = 700.0 * progress as f32 / total as f32;
        gfx.fill_rect(
            &Rectangle::new(Vector::new(50.0, 500.0), Vector::new(width, 25.0)),
            Color::BLUE,
        );

        gfx.present(&window)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let settings = Settings {
        title: "TakiRouge",
        size: DEFAULT_WINDOW_SIZE,
        // resizable: true,
        ..Settings::default()
    };
    run(settings, lib::app)
}
