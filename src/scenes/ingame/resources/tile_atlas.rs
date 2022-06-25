use bevy::prelude::*;

use super::Tile;

pub struct TileAtlas {
    textures: Vec<Handle<Image>>,
}

impl TileAtlas {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        let texture_terrain = asset_server.load("textures/ground_terrain.png");
        let texture_water = asset_server.load("textures/ground_water.png");
        let texture_other = asset_server.load("textures/ground_other.png");

        let textures = vec![texture_terrain, texture_water,texture_other];

        Self { textures }
    }

    pub fn tile_of_type(&self, mut tile_type: usize) -> Tile {

        if tile_type > self.textures.len() {
            tile_type = 0;
        }

        Tile::new(self.textures[tile_type].clone())
    }
}
