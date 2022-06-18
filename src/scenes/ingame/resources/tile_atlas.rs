use bevy::prelude::*;

use super::Tile;

pub struct TileAtlas {
    textures: Vec<Handle<Image>>,
}

impl TileAtlas {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        let texture = asset_server.load("textures/rainbow_island.png");

        let textures = vec![texture];

        Self { textures }
    }

    pub fn tile_of_type(&self, tile_type: usize) -> Tile {
        Tile::new(self.textures[tile_type].clone())
    }

    pub fn tile_types(&self) -> usize {
        self.textures.len()
    }
}
