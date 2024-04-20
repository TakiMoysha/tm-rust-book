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
            Color, Element, Graphics, Image, Mesh, PixelFormat, Surface, Vertex,
        },
        Input, Result, Settings, Window,
    };

    fn draw_atlas(gfx: &mut Graphics, image: Image) {
        gfx.clear(Color::WHITE);
        gfx.set_blend_mode(Some(BlendMode {
            equation: Default::default(),
            function: BlendFunction::Same {
                source: BlendFactor::Color {
                    input: BlendInput::Source,
                    channel: BlendChannel::Alpha,
                    is_inverse: false,
                },
                destination: BlendFactor::Color {
                    input: BlendInput::Source,
                    channel: BlendChannel::Alpha,
                    is_inverse: true,
                },
            },
            global_color: [0.0; 4],
        }));

        let overlay_region = Rectangle::new(Vector::new(0.0, 0.0), image.size());
        gfx.draw_image(&image, overlay_region);
        gfx.fill_rect(&overlay_region, Color::BLUE.with_alpha(0.3));

        gfx.set_blend_mode(Some(BlendMode {
            equation: Default::default(),
            function: BlendFunction::Same {
                source: BlendFactor::One,
                destination: BlendFactor::Zero,
            },
            global_color: [0.0; 4],
        }));

        let region = Rectangle::new(Vector::new(0.0, 0.0), Vector::new(100.0, 40.0));
        gfx.draw_image(&image, region);
        gfx.fill_rect(&region, Color::RED.with_alpha(0.1));
    }

    pub async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
        let img_atlas = Image::load(&gfx, "assets/atlas.png").await?;

        draw_atlas(&mut gfx, img_atlas);

        gfx.present(&window)?;

        loop {
            while let Some(_) = input.next_event().await {}
        }
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
