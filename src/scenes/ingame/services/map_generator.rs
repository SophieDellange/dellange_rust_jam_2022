use rand::Rng;

use crate::scenes::ingame::resources::{Map, TileAtlas};

pub struct MapGenerator {
    tile_atlas: TileAtlas,
}

impl MapGenerator {
    pub fn new(tile_atlas: TileAtlas) -> Self {
        Self { tile_atlas }
    }

    pub fn build_map(&self, width: u16, height: u16) -> Map {
        let tiles = (0..height)
            .map(|_| {
                (0..width)
                    .map(|_| {
                        let tile_type =
                            rand::thread_rng().gen_range(0..self.tile_atlas.tile_types());
                        self.tile_atlas.tile_of_type(tile_type)
                    })
                    .collect()
            })
            .collect();

        Map { tiles }
    }
}
