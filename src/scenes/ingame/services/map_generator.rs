use rand::{thread_rng, Rng};

use crate::scenes::ingame::resources::{Map, TileAtlas};

pub struct MapGenerator {
    tile_atlas: TileAtlas,
    // Size in tiles
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

                        let tile_type = match  thread_rng().gen_range(0..=100){
                            i32::MIN..=-1_i32 | 101_i32..=i32::MAX  | 0_i32..=90 => 0,
                            91..=95 => 1,
                            96..=100 => 2,
                        };
                        self.tile_atlas.tile_of_type(tile_type)
                    })
                    .collect()
            })
            .collect();

        Map { tiles }
    }
}
