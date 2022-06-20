use bevy::utils::Duration;
use bevy::{math::const_vec2, prelude::*};

use super::{BlockData, Collider};

const PLAYER_Z: f32 = 1.0;
const PLAYER_SIZE: Vec2 = const_vec2!([64., 64.]);

#[derive(Component)]
pub struct Player {
    pub firing_clock: Timer,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl Player {
    pub fn new() -> Self {
        Player {
            firing_clock: Timer::new(Duration::from_secs_f32(0.3), true),
        }
    }

    pub fn spawn(&self, location: Vec2, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let player = Player::new();

        let texture = asset_server.load("textures/block_core.png");

        let sprite_bundle = SpriteBundle {
            texture: texture,
            transform: Transform::from_xyz(location.x, location.y, PLAYER_Z),
            sprite: Sprite {
                custom_size: Some(PLAYER_SIZE),
                ..Default::default()
            },
            ..default()
        };

        let player_bundle = PlayerBundle {
            player,
            sprite_bundle,
        };

        commands
            .spawn_bundle(player_bundle)
            .insert(Collider)
            .insert(BlockData {
                health: 20,
                ..default()
            });
    }
}
