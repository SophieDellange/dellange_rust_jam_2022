use rand::Rng;

use crate::scenes::ingame::resources::{Map, TileAtlas};

pub struct MapGenerator {
    tile_atlas: TileAtlas,
    map_width: u16,
    map_height: u16,
}

impl MapGenerator {
    pub fn new(tile_atlas: TileAtlas, width: u16, height: u16) -> Self {
        Self {
            tile_atlas,
            map_width: width,
            map_height: height,
        }
    }

    pub fn build_map(&self) -> Map {
        let tiles = (0..self.map_height)
            .map(|_| {
                (0..self.map_width)
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
