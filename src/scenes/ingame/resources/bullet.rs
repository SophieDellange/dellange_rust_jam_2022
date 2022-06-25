// This is due to #[derive(Bundle)]  issue https://github.com/bevyengine/bevy/issues/4601
#![allow(clippy::forget_non_drop)]

use bevy::{
    math::const_vec2, prelude::*, render::camera::Camera2d, sprite::collide_aabb::collide,
    utils::Duration,
};
use rand::prelude::SliceRandom;

use crate::scenes::ingame::resources::player_core_tile::PlayerCoreTile;

use super::{Enemy, Player};

pub const BULLET_SIZE: Vec2 = const_vec2!([10., 10.]);
pub const BULLET_SPEED: f32 = 14.;

pub const ENEMY_BULLET_INTERVAL: f32 = 1.;

// If the enemy bullet timer is not shared, it could be a Local, however, preinitializing a local seems
// to be very clunky (when a system has a large signature).
pub struct EnemyBulletTimer(pub Timer);

#[derive(Component)]
pub struct Bullet {
    texture: Handle<Image>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Component)]
pub struct BulletItem {
    direction: Vec2,
    speed: f32,
    life_time: Timer,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Bundle)]
pub struct BulletBundle<C: Component> {
    owner: C,
    bullet_item: BulletItem,
}

impl<C: Component> BulletBundle<C> {
    pub fn new(owner: C, bullet_item: BulletItem) -> Self {
        Self { owner, bullet_item }
    }
}

impl Bullet {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        let texture = asset_server.load("textures/laserGreen1.png");
        Self { texture }
    }

    pub fn spawn<C: Component>(
        &self,
        location: &Transform,
        direction: Vec2,
        owner: C,
        commands: &mut Commands,
    ) {
        // Starts from the producer position
        let mut new_transf = *location;
        //let spawn_loc = new_transf.translation.truncate() + direction;

        //new_transf.translation = Vec3::new(spawn_loc.x, spawn_loc.x, location.translation.z);
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
            .insert_bundle(BulletBundle::new(
                owner,
                BulletItem {
                    direction,
                    speed: BULLET_SPEED,
                    life_time: Timer::new(Duration::from_secs_f32(2.0), false),
                },
            ));
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

pub fn spawn_player_bullets(
    mut head: Query<(&Transform, &mut PlayerCoreTile)>,
    server: Res<AssetServer>,
    time: Res<Time>,
    mut commands: Commands,
) {
    if let Ok((spawn_location, mut player)) = head.get_single_mut() {
        player.firing_clock.tick(time.delta());
        if player.firing_clock.finished() {
            let bullet = Bullet::new(&server);
            bullet.spawn(
                spawn_location,
                Vec2::new(1.0, 0.0),
                Player::new(),
                &mut commands,
            );
        }
    }
}

pub fn spawn_enemy_bullets(
    mut commands: Commands,
    q_camera: Query<&GlobalTransform, With<Camera2d>>,
    q_enemies: Query<(&Transform, &Sprite), With<Enemy>>,
    windows: Res<Windows>,
    mut enemy_bullet_timer: ResMut<EnemyBulletTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    let camera_location = q_camera.single().translation;
    let window = windows.get_primary().unwrap();
    let camera_size = Vec2::new(window.width(), window.height());

    let on_screen_enemy_locations = q_enemies
        .iter()
        .filter_map(|(enemy_transform, enemy_sprite)| {
            let enemy_location = enemy_transform.translation;
            let enemy_size = enemy_sprite.custom_size.unwrap();

            collide(enemy_location, enemy_size, camera_location, camera_size)
                .map(|_| enemy_transform)
        })
        .collect::<Vec<_>>();

    enemy_bullet_timer.0.tick(time.delta());

    if enemy_bullet_timer.0.finished() {
        let enemy_location = on_screen_enemy_locations.choose(&mut rand::thread_rng());

        if let Some(enemy_location) = enemy_location {
            let bullet = Bullet::new(&asset_server);

            bullet.spawn(
                enemy_location,
                Vec2::new(-1.0, 0.0),
                Enemy::new(),
                &mut commands,
            );
        }

        enemy_bullet_timer.0.reset();
    }
}
