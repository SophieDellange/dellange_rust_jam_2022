use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::scenes::ingame::resources::{Enemy, Map, TileAtlas, TILE_SIZE};

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
                        let tile_type = thread_rng().gen_range(0..self.tile_atlas.tile_types());
                        self.tile_atlas.tile_of_type(tile_type)
                    })
                    .collect()
            })
            .collect();

        Map { tiles }
    }

    pub fn generate_enemies(
        &self,
        count: u8,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) {
        for _ in 0..count {
            let location = Vec2::new(
                thread_rng().gen_range(0..(self.map_width * TILE_SIZE.x as u16)) as f32,
                -(thread_rng().gen_range(0..(self.map_width * TILE_SIZE.x as u16)) as f32),
            );

            Enemy::new(asset_server).spawn(location, commands);
        }
    }
}
