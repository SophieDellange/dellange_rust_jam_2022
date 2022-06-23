use bevy::prelude::*;

use super::{
    player_core_tile::{PLAYER_TILE_SIZE, PLAYER_TILE_Z},
    BlockData, LootType, Player,
};

#[derive(Component)]
pub struct PlayerExtraTile {}

#[derive(Bundle)]
struct PlayerExtraTileBundle {
    tile: PlayerExtraTile,
    player: Player,
    stats: BlockData,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl PlayerExtraTile {
    pub fn new() -> Self {
        Self {}
    }

    pub fn spawn(
        &self,
        loot_type: &LootType,
        location: Vec2,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) {
        let tile = PlayerExtraTile::new();
        let player = Player::new();

        let texture = loot_type.player_extra_tile_texture(asset_server);

        let sprite_bundle = SpriteBundle {
            texture: texture,
            transform: Transform::from_xyz(location.x, location.y, PLAYER_TILE_Z),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_TILE_SIZE, PLAYER_TILE_SIZE)),
                ..Default::default()
            },
            ..default()
        };

        let player_bundle = PlayerExtraTileBundle {
            tile,
            player,
            sprite_bundle,
            stats: BlockData::new(10),
        };

        commands.spawn_bundle(player_bundle);
    }
}
