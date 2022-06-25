// This is due to #[derive(Bundle)]  issue https://github.com/bevyengine/bevy/issues/4601
#![allow(clippy::forget_non_drop)]

use bevy::prelude::*;
use rand::{thread_rng, Rng};

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
    pub fn spawn(
        loot_type: &LootType,
        location: Vec2,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) {
        let tile = PlayerExtraTile {};
        let player = Player::new();

        let collider = Collider {};
        let block_data = BlockData::new(EXTRA_TILE_HEALTH);

        let texture = loot_type.player_extra_tile_texture(asset_server);
        let mut transform =Transform::from_xyz(location.x, location.y, PLAYER_TILE_Z);

        let rotation_amt = match thread_rng().gen_range(0..=3) {
            i32::MIN..=-1_i32 | 3_i32..=i32::MAX => 0.0 ,
            0_i32 => std::f32::consts::FRAC_PI_2,
            1 => std::f32::consts::PI,
            2 => std::f32::consts::FRAC_PI_2 * 2.,
        };

        transform.rotate(Quat::from_rotation_z(rotation_amt));

        let sprite_bundle = SpriteBundle {
            texture,
            transform ,
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
