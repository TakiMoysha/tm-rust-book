use quicksilver::run;
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics},
    Input, Result, Settings, Window,
};

pub mod lib {
    use quicksilver::{
        geom::{Circle, Rectangle, Vector},
        graphics::{
            blend::{BlendChannel, BlendFactor, BlendFunction, BlendInput, BlendMode},
            Color, Element, FontRenderer, Graphics, Image, Mesh, PixelFormat, Surface, VectorFont,
            Vertex,
        },
        input::{Event, Key},
        Input, Result, Settings, Timer, Window,
    };

    fn draw_atlas(gfx: &mut Graphics, image: &Image) {
        gfx.clear(Color::WHITE);
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
        let asset_atlas = Image::load(&gfx, "assets/atlas.png").await?;
        let asset_font = VectorFont::load("assets/amethysta.ttf").await?;

        // draw atlas
        draw_atlas(&mut gfx, &asset_atlas);

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
        gfx.present(&window)?;

        // let mut update_timer = Timer::time_per_second(30.0);
        // let mut draw_timer = Timer::time_per_second(60.0);
        // let mut rect = Rectangle::new(Vector::new(0.0, 80.0), Vector::new(16.0, 16.0));

        let mut square_position = Vector::new(300.0, 300.0);
        let mut my_string = String::new();
        let mut is_running = true;

        while is_running {
            while let Some(event) = input.next_event().await {
                match event {
                    Event::KeyboardInput(input_event) => {
                        if input_event.key() == Key::Escape {
                            is_running = false;
                        } else if input_event.key() == Key::Back {
                            my_string.pop();
                        }
                    }
                    Event::ReceivedCharacter(c) => {
                        let chr = c.character();
                        if !chr.is_control() { my_string.push(chr); }
                    }
                    _ => {}
                }
            }

            // if input.key_down(Key::H) {}
            // if input.key_down(Key::J) {}
            // if input.key_down(Key::K) {}
            // if input.key_down(Key::L) {}

            // gfx.clear(Color::WHITE);
            font.draw_wrapping(&mut gfx, &my_string, Some(500.0), Color::BLACK, Vector::new(100.0, 100.0))?;
            gfx.present(&window)?;
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    // let font = "amethysta.ttf";
    let settings = Settings {
        title: "TakiRouge",
        size: Vector::new(1200.0, 600.0),
        ..Settings::default()
    };
    run(settings, lib::app)
}
