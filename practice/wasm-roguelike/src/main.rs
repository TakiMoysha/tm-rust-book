use quicksilver::run;
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics},
    Input, Result, Settings, Window,
};

pub mod lib {
    use quicksilver::{
        geom::{Circle, Rectangle, Vector},
        graphics::{Color, Element, Graphics, Image, Mesh, PixelFormat, Surface, Vertex},
        Input, Result, Settings, Window,
    };

    pub async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
        gfx.clear(Color::BLACK);

        let texture = Image::from_raw(&gfx, None, 512, 512, PixelFormat::RGBA)?;
        let mut surface = Surface::new(&gfx, texture)?;

        gfx.fill_rect(
            &Rectangle::new(Vector::new(0.0, 0.0), Vector::new(100.0, 100.0)),
            Color::RED,
        );
        gfx.fill_circle(&Circle::new(Vector::new(400.0, 150.0), 50.0), Color::WHITE);
        gfx.flush_surface(&surface)?;
        gfx.clear(Color::BLACK);

        let image = surface.detach().expect("The image failed to detach!");

        gfx.draw_image(&image, Rectangle::new_sized(Vector::new(400.0, 300.0)));
        gfx.draw_image(
            &image,
            Rectangle::new(Vector::new(400.0, 400.0), Vector::new(200.0, 200.0)),
        );
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
