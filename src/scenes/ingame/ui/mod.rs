use bevy::prelude::*;

use super::resources::Player;

#[derive(Component)]
pub struct HealthBar;

#[derive(Bundle)]
pub struct HealthBarBundle {
    bar: HealthBar,

    #[bundle]
    sprite_bundle: SpriteBundle,
}

pub fn spawn_health_bar(
    mut commands: Commands,
    q_player: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("textures/green_bar.png");

    let mut player_position = q_player.single().clone();
    player_position.translation += Vec3::from((0.0, -4.0, 0.0));

    let sprite_bundle = SpriteBundle {
        texture: texture,
        transform: player_position.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2::new(60.0, 60.0)),
            ..Default::default()
        },
        ..default()
    };

    let health_bar_bundle = HealthBarBundle {
        bar: HealthBar {},
        sprite_bundle,
    };

    commands.spawn_bundle(health_bar_bundle);
}

pub fn update_health_bar() {}
