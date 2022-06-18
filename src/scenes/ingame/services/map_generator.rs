use rand::Rng;

use crate::scenes::ingame::resources::{Map, Tile, TILE_TYPE_COUNT};

pub struct MapGenerator {}

impl MapGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build_map(&self, width: u16, height: u16) -> Map {
        let tiles = (0..height)
            .map(|_| {
                (0..width)
                    .map(|_| {
                        let tile_type = rand::thread_rng().gen_range(0..TILE_TYPE_COUNT);
                        Tile::of_type(tile_type)
                    })
                    .collect()
            })
            .collect();

        Map { tiles }
    }
}
