use bevy::{math::const_vec2, prelude::*};

use super::{BlockData, Collider, Enemy};

const ENEMIES_Z: f32 = 1.0;
const ENEMIES_SIZE: Vec2 = const_vec2!([64., 64.]);

#[derive(Bundle)]
pub struct EnemyBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    collider: Collider,
    block_data: BlockData,
    enemy: Enemy,
}

impl EnemyBundle {
    pub fn spawn(location: Vec2, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let texture = asset_server.load("textures/enemy_barnacle.png");

        let sprite_bundle = SpriteBundle {
            texture: texture,
            transform: Transform::from_xyz(location.x, location.y, ENEMIES_Z),
            sprite: Sprite {
                custom_size: Some(ENEMIES_SIZE),
                ..Default::default()
            },
            ..default()
        };

        let collider = Collider {};
        let block_data = BlockData::new(12);
        let enemy = Enemy::new();

        let enemy_bundle = Self {
            sprite_bundle,
            collider,
            block_data,
            enemy,
        };

        commands.spawn_bundle(enemy_bundle);
    }
}
