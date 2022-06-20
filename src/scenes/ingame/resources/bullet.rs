use bevy::{math::const_vec2, prelude::*, utils::Duration};

use crate::scenes::ingame::resources::player_tile::PlayerTile;

const BULLET_SIZE: Vec2 = const_vec2!([6., 6.]);

#[derive(Component)]
pub struct Bullet {
    texture: Handle<Image>,
}

#[derive(Component)]
pub struct BulletItem {
    direction: Vec2,
    speed: f32,
    life_time: Timer,
}

impl Bullet {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        let texture = asset_server.load("textures/laserGreen1.png");
        Self { texture }
    }

    pub fn spawn(&self, location: &Transform, direction: Vec2, commands: &mut Commands) {
        let mut new_transf = location.clone();
        new_transf.rotate(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2));

        commands
            .spawn_bundle(SpriteBundle {
                texture: self.texture.clone(),
                transform: new_transf,
                sprite: Sprite {
                    custom_size: Some(BULLET_SIZE),
                    ..Default::default()
                },
                ..default()
            })
            .insert(BulletItem {
                direction,
                speed: 14.0,
                life_time: Timer::new(Duration::from_secs_f32(2.0), false),
            });
    }
}

// System to move bullets in their direction (should support any direction/speed)
pub fn move_bullets(
    mut query: Query<(&mut Transform, &mut BulletItem, Entity)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut transform, mut bullet, entity) in query.iter_mut() {
        bullet.life_time.tick(time.delta());
        if bullet.life_time.finished() {
            commands.entity(entity).despawn();
        } else {
            transform.translation.x += bullet.direction.x * bullet.speed;
            transform.translation.y += bullet.direction.y * bullet.speed;
        }
    }
}

pub fn spawn_bullets(
    mut head: Query<(&Transform, &mut PlayerTile)>,
    server: Res<AssetServer>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (spawn_location, mut player) in head.iter_mut() {
        player.firing_clock.tick(time.delta());
        if player.firing_clock.finished() {
            let bullet = Bullet::new(&server);
            bullet.spawn(spawn_location, Vec2::new(1.0, 0.0), &mut commands);
        }
    }
}
