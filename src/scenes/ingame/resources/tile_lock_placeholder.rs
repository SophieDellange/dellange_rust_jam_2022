use bevy::prelude::*;

use super::PLAYER_TILE_SIZE;

const LOCK_TILE_Z: f32 = 0.5;

#[derive(Component)]
pub struct TicketLockPlaceholder {}

#[derive(Bundle)]
struct TicketLockPlaceholderBundle {
    player: TicketLockPlaceholder,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl TicketLockPlaceholder {
    pub fn new() -> Self {
        TicketLockPlaceholder {}
    }

    pub fn spawn(&self, location: Vec2, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let player = TicketLockPlaceholder::new();

        let texture = asset_server.load("textures/tile_lock_placeholder.png");

        let sprite_bundle = SpriteBundle {
            texture: texture,
            transform: Transform::from_xyz(location.x, location.y, LOCK_TILE_Z),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_TILE_SIZE, PLAYER_TILE_SIZE)),
                ..Default::default()
            },
            ..default()
        };

        let player_bundle = TicketLockPlaceholderBundle {
            player,
            sprite_bundle,
        };

        commands.spawn_bundle(player_bundle);
    }
}
