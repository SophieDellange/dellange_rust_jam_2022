// This is due to #[derive(Bundle)]  issue https://github.com/bevyengine/bevy/issues/4601
#![allow(clippy::forget_non_drop)]

use bevy::prelude::*;

use super::player_core_tile::PLAYER_TILE_SIZE;

const LOCK_TILE_Z: f32 = 0.5;

#[derive(Component)]
pub struct TileLock {}

#[derive(Bundle)]
struct TileLockBundle {
    player: TileLock,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl TileLock {
    pub fn new() -> Self {
        TileLock {}
    }

    pub fn spawn(location: Vec2, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let player = TileLock::new();

        let texture = asset_server.load("textures/tile_lock.png");

        let sprite_bundle = SpriteBundle {
            texture,
            transform: Transform::from_xyz(location.x, location.y, LOCK_TILE_Z),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_TILE_SIZE, PLAYER_TILE_SIZE)),
                ..Default::default()
            },
            ..default()
        };

        let player_bundle = TileLockBundle {
            player,
            sprite_bundle,
        };

        commands.spawn_bundle(player_bundle);
    }
}
