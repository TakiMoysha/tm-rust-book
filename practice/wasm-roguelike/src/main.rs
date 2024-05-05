use wasm_roguelike;

use quicksilver::run;
use quicksilver::{geom::Vector, Result, Settings};

const DEFAULT_WINDOW_SIZE: Vector = Vector { x: 800.0, y: 600.0 };

fn main() -> Result<()> {
    let settings = Settings {
        title: "TakiRouge",
        size: DEFAULT_WINDOW_SIZE,
        // resizable: true,
        ..Settings::default()
    };
    run(settings, wasm_roguelike::app)
}
