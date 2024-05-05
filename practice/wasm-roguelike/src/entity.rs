use quicksilver::{geom::Vector, graphics::Color};

#[derive(Clone, Debug, PartialEq)]
pub struct Entity {
    pub id: usize,
    pub pos: Vector,
    pub glyph: char,
    pub color: Color,
    pub hp: i32,
    pub max_hp: i32,
}

pub fn generate_entities() -> Vec<Entity> {
    vec![
        Entity {
            id: 1,
            pos: Vector::new(1.0, 1.0),
            glyph: 'H',
            color: Color::GREEN,
            hp: 10,
            max_hp: 10,
        },
        Entity {
            id: 2,
            pos: Vector::new(9.0, 6.0),
            glyph: 'g',
            color: Color::GREEN,
            hp: 1,
            max_hp: 1,
        },
        Entity {
            id: 3,
            pos: Vector::new(2.0, 4.0),
            glyph: 'g',
            color: Color::CYAN,
            hp: 1,
            max_hp: 3,
        },
        Entity {
            id: 4,
            pos: Vector::new(7.0, 5.0),
            glyph: '%',
            color: Color::BLUE,
            hp: 0,
            max_hp: 0,
        },
        Entity {
            id: 5,
            pos: Vector::new(4.0, 8.0),
            glyph: '%',
            color: Color::BLUE,
            hp: 0,
            max_hp: 0,
        },
    ]
}

pub fn create_player() -> Entity {
    Entity {
        id: 0,
        pos: Vector::new(5.0, 5.0),
        glyph: '@',
        color: Color::MAGENTA,
        hp: 3,
        max_hp: 10,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_fields() {
        let player = create_player();
        assert_eq!(player.id, 0);
        assert_eq!(player.pos, Vector::new(5.0, 5.0));
        assert_eq!(player.glyph, '@');
        assert_eq!(player.color, Color::MAGENTA);
        assert_eq!(player.hp, 3);
        assert_eq!(player.max_hp, 10);
    }
}
