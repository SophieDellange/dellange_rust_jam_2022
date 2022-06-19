use bevy::{math::const_vec2, prelude::*};

const BULLET_Z: f32 = 0.3;
const BULLET_SIZE: Vec2 = const_vec2!([6., 6.]);

#[derive(Component)]
pub struct Bullet {
    texture: Handle<Image>,
    direction: Option<Vec3>,
}

impl Bullet {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        let texture = asset_server.load("textures/laserGreen1.png");
        Self {
            texture,
            direction: None,
        }
    }

    pub fn spawn(&self, location: Vec2, direction: Vec3, commands: &mut Commands) {
        commands.spawn_bundle(SpriteBundle {
            texture: self.texture.clone(),
            transform: Transform::from_xyz(location.x, location.y, BULLET_SIZE),
            sprite: Sprite {
                custom_size: Some(BULLET_Z),
                ..Default::default()
            },
            ..default()
        });
    }
}
const TIME_STEP: f32 = 1.0 / 60.0;

pub fn move_bullets(mut query: Query<(&mut Transform, &Bullet)>) {
    for (mut transform, bullet) in query.iter_mut() {
        if let Some(direction) = bullet.direction {
            transform.translation.x += direction.x * TIME_STEP;
            transform.translation.y += direction.y * TIME_STEP;
        }
    }
}

pub fn spawn_bullets(head: Query<&HeadMeat>, mut commands: Commands) {}
