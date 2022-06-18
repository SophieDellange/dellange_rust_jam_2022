use bevy::prelude::Color;

pub const TILE_TYPE_COUNT: u8 = 7;

pub struct Tile {
    pub color: Color,
}

impl Tile {
    pub fn of_type(tile_type: u8) -> Self {
        if tile_type > TILE_TYPE_COUNT {
            panic!("type_type > {}", TILE_TYPE_COUNT);
        }

        let unit = 1. / (TILE_TYPE_COUNT as f32);

        let color = Color::Rgba {
            red: tile_type as f32 * unit,
            green: tile_type as f32 * unit,
            blue: tile_type as f32 * unit,
            alpha: 0.,
        };

        Self { color }
    }
}
