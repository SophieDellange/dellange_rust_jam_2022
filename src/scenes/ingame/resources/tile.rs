use bevy::prelude::*;

pub const TILE_SIZE: f32 = 64.; // pixels
pub const TILES_Z: f32 = 0.;

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

    pub fn spawn(&self, location: Vec2, commands: &mut Commands) {
        commands.spawn_bundle(SpriteBundle {
            texture: self.texture(),
            transform: Transform::from_xyz(location.x, location.y, TILES_Z),
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
            ..default()
        });
    }
}
