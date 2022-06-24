// This is due to #[derive(Bundle)]  issue https://github.com/bevyengine/bevy/issues/4601
#![allow(clippy::forget_non_drop)]

use bevy::prelude::*;

use super::{
    player_core_tile::{PLAYER_TILE_SIZE, PLAYER_TILE_Z},
    BlockData, Collider, LootType, Player,
};

const EXTRA_TILE_HEALTH: u8 = 1;

#[derive(Component)]
pub struct PlayerExtraTile {}

#[derive(Bundle)]
struct PlayerExtraTileBundle {
    tile: PlayerExtraTile,
    player: Player,
    collider: Collider,
    block_data: BlockData,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl PlayerExtraTile {
    pub fn new() -> Self {
        Self {}
    }

    pub fn spawn(
        loot_type: &LootType,
        location: Vec2,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) {
        let tile = PlayerExtraTile::new();
        let player = Player::new();

        let collider = Collider {};
        let block_data = BlockData::new(EXTRA_TILE_HEALTH);

        let texture = loot_type.player_extra_tile_texture(asset_server);

        let sprite_bundle = SpriteBundle {
            texture,
            transform: Transform::from_xyz(location.x, location.y, PLAYER_TILE_Z),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_TILE_SIZE, PLAYER_TILE_SIZE)),
                ..default()
            },
            ..default()
        };

        let player_bundle = PlayerExtraTileBundle {
            tile,
            player,
            collider,
            block_data,
            sprite_bundle,
        };

        commands.spawn_bundle(player_bundle);
    }
}
