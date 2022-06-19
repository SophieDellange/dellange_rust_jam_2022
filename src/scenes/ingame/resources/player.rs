use bevy::{math::const_vec2, prelude::*};

const PLAYER_Z: f32 = 1.0;
const PLAYER_SIZE: Vec2 = const_vec2!([64., 64.]);

#[derive(Component)]
pub struct Player {}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl Player {
    pub fn new() -> Self {
        Player {}
    }

    pub fn spawn(&self, location: Vec2, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let player = Player::new();

        let texture = asset_server.load("textures/player_brick_crate.png");

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

        commands.spawn_bundle(player_bundle);
    }
}
