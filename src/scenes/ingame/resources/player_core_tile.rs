// This is due to #[derive(Bundle)]  issue https://github.com/bevyengine/bevy/issues/4601
#![allow(clippy::forget_non_drop)]

use bevy::prelude::*;
use bevy::utils::Duration;

use super::{BlockData, Collider, Player};

pub const PLAYER_TILE_Z: f32 = 1.0;
pub const PLAYER_TILE_SIZE: f32 = 64.;

#[derive(Component, Debug)]
pub struct PlayerCoreTile {
    pub firing_clock: Timer,
}

#[derive(Bundle)]
struct PlayerCoreTileBundle {
    tile: PlayerCoreTile,
    player: Player,
    collider: Collider,
    stats: BlockData,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl PlayerCoreTile {
    pub fn new() -> Self {
        PlayerCoreTile {
            firing_clock: Timer::new(Duration::from_secs_f32(0.3), true),
        }
    }

    pub fn spawn(location: Vec2, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let tile = PlayerCoreTile::new();
        let player = Player::new();

        let texture = asset_server.load("textures/creature_hearth.png");

        let sprite_bundle = SpriteBundle {
            texture,
            transform: Transform::from_xyz(location.x, location.y, PLAYER_TILE_Z),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_TILE_SIZE, PLAYER_TILE_SIZE)),
                ..Default::default()
            },
            ..default()
        };

        let collider = Collider {};

        let player_bundle = PlayerCoreTileBundle {
            tile,
            player,
            sprite_bundle,
            collider,
            stats: BlockData::new(20),
        };

        commands.spawn_bundle(player_bundle);
    }
}
