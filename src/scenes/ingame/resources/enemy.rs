use bevy::{math::const_vec2, prelude::*};

const ENEMIES_Z: f32 = 1.0;
const ENEMIES_SIZE: Vec2 = const_vec2!([64., 64.]);

#[derive(Component)]
pub struct Enemy {
    texture: Handle<Image>,
}

impl Enemy {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        let texture = asset_server.load("textures/enemy_bee.png");

        Self { texture }
    }

    pub fn spawn(&self, location: Vec2, commands: &mut Commands) {
        commands.spawn_bundle(SpriteBundle {
            texture: self.texture.clone(),
            transform: Transform::from_xyz(location.x, location.y, ENEMIES_Z),
            sprite: Sprite {
                custom_size: Some(ENEMIES_SIZE),
                ..Default::default()
            },
            ..default()
        });
    }
}
