use quicksilver::{geom::Vector, graphics::Color};

#[derive(Clone, Debug, PartialEq)]
pub struct Tile {
    pos: Vector,
    glyph: char,
    color: Color,
}

pub fn generate_map(size: Vector, level: i32) -> Vec<Tile> {
    let width = size.x as usize;
    let height = size.y as usize;
    let mut map = Vec::with_capacity(width * height);

    for x in 0..width {
        for y in 0..height {
            let mut tile = Tile {
                pos: Vector::new(x as f32, y as f32),
                glyph: '.',
                color: Color::BLACK,
            };

            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                tile.glyph = '#';
                tile.color = Color::RED;
            };
            map.push(tile);
        }
    }
    map
}
