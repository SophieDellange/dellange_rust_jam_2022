use bevy::prelude::*;

pub struct Tile {
    texture: Handle<Image>,
}

impl Tile {
    pub fn new(texture: Handle<Image>) -> Self {
        Self { texture }
    }

    pub fn texture(&self) -> Handle<Image> {
        self.texture.clone()
    }
}
