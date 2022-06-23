use std::f32::consts::PI;

use bevy::{math::const_vec2, prelude::*};
use rand::{thread_rng, Rng};

use super::{BlockData, Collider, Enemy};

const MIN_MOVEMENT_TIMER: f32 = 0.5;
const MAX_MOVEMENT_TIMER: f32 = 2.0;

const ENEMIES_Z: f32 = 1.0;
const ENEMIES_SIZE: Vec2 = const_vec2!([64., 64.]);
pub const ENEMIES_SPEED: f32 = 2.5;

#[derive(Component)]
pub struct RandomMovement {
    pub timer: Timer,
    pub direction: Vec3,
}

impl RandomMovement {
    fn new() -> Self {
        let timer_duration = thread_rng().gen_range(MIN_MOVEMENT_TIMER..MAX_MOVEMENT_TIMER);

        let mut instance = Self {
            timer: Timer::from_seconds(timer_duration, false),
            direction: Vec3::ZERO,
        };

        instance.renew();

        instance
    }

    pub fn renew(&mut self) {
        self.timer.reset();
        let angle = thread_rng().gen_range(0_f32..(2_f32 * PI));
        self.direction = Vec3::new(angle.cos(), angle.sin(), 0.);
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    collider: Collider,
    block_data: BlockData,
    enemy: Enemy,
    movement: RandomMovement,
}

impl EnemyBundle {
    pub fn spawn(location: Vec2, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let texture = asset_server.load("textures/enemy_barnacle.png");

        let sprite_bundle = SpriteBundle {
            texture: texture,
            transform: Transform::from_xyz(location.x, location.y, ENEMIES_Z),
            sprite: Sprite {
                custom_size: Some(ENEMIES_SIZE),
                ..Default::default()
            },
            ..default()
        };

        let collider = Collider {};
        let block_data = BlockData::new(12);
        let enemy = Enemy::new();

        let movement = RandomMovement::new();

        let enemy_bundle = Self {
            sprite_bundle,
            collider,
            block_data,
            enemy,
            movement,
        };

        commands.spawn_bundle(enemy_bundle);
    }
}
