use bevy::{math::const_vec2, prelude::*};

const PLAYER_Z: f32 = 1.0;
const PLAYER_SIZE: Vec2 = const_vec2!([64., 64.]);

#[derive(Component)]
pub struct Player {
    texture: Handle<Image>,
}

impl Player {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        let texture = asset_server.load("textures/player_brick_crate.png");

        Self { texture }
    }

    pub fn spawn(&self, location: Vec2, commands: &mut Commands) {
        commands.spawn_bundle(SpriteBundle {
            texture: self.texture.clone(),
            transform: Transform::from_xyz(location.x, location.y, PLAYER_Z),
            sprite: Sprite {
                custom_size: Some(PLAYER_SIZE),
                ..Default::default()
            },
            ..default()
        });
    }
}
