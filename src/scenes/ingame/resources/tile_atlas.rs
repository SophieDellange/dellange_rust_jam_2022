use bevy::prelude::*;

use super::Tile;

pub struct TileAtlas {
    textures: Vec<Handle<Image>>,
}

impl TileAtlas {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        let texture_terrain = asset_server.load("textures/ground_terrain.png");
        let texture_water = asset_server.load("textures/ground_water.png");

        let textures = vec![texture_terrain, texture_water];

        Self { textures }
    }

    pub fn tile_of_type(&self, tile_type: usize) -> Tile {
        Tile::new(self.textures[tile_type].clone())
    }

    pub fn tile_types(&self) -> usize {
        self.textures.len()
    }
}
