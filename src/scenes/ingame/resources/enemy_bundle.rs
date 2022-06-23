use bevy::{math::const_vec2, prelude::*};

use super::{BlockData, Collider, Enemy};

const ENEMIES_Z: f32 = 1.0;
const ENEMIES_SIZE: Vec2 = const_vec2!([64., 64.]);

#[derive(Component)]
pub struct EnemyBundle {
    texture: Handle<Image>,
}

impl EnemyBundle {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        let texture = asset_server.load("textures/enemy_barnacle.png");

        Self { texture }
    }

    pub fn spawn(&self, location: Vec2, commands: &mut Commands) {
        commands
            .spawn_bundle(SpriteBundle {
                texture: self.texture.clone(),
                transform: Transform::from_xyz(location.x, location.y, ENEMIES_Z),
                sprite: Sprite {
                    custom_size: Some(ENEMIES_SIZE),
                    ..Default::default()
                },
                ..default()
            })
            .insert(Collider {})
            .insert(BlockData::new(12))
            .insert(Enemy::new());
    }
}
