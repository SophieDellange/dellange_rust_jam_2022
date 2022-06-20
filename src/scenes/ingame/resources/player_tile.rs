use bevy::utils::Duration;
use bevy::{math::const_vec2, prelude::*};

const PLAYER_TILE_Z: f32 = 1.0;
const PLAYER_TILE_SIZE: Vec2 = const_vec2!([64., 64.]);

#[derive(Component)]
pub struct PlayerTile {
    pub firing_clock: Timer,
}

#[derive(Bundle)]
struct PlayerTileBundle {
    player: PlayerTile,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl PlayerTile {
    pub fn new() -> Self {
        PlayerTile {
            firing_clock: Timer::new(Duration::from_secs_f32(0.3), true),
        }
    }

    pub fn spawn(&self, location: Vec2, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let player = PlayerTile::new();

        let texture = asset_server.load("textures/block_core.png");

        let sprite_bundle = SpriteBundle {
            texture: texture,
            transform: Transform::from_xyz(location.x, location.y, PLAYER_TILE_Z),
            sprite: Sprite {
                custom_size: Some(PLAYER_TILE_SIZE),
                ..Default::default()
            },
            ..default()
        };

        let player_bundle = PlayerTileBundle {
            player,
            sprite_bundle,
        };

        commands.spawn_bundle(player_bundle);
    }
}
