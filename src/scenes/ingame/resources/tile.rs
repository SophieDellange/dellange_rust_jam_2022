use bevy::{math::const_vec2, prelude::*};

pub const TILE_SIZE: Vec2 = const_vec2!([64., 64.]); // pixels
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
                custom_size: Some(TILE_SIZE),
                ..Default::default()
            },
            ..default()
        });
    }
}
