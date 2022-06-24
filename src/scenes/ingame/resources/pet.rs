use bevy::{math::const_vec2, prelude::*};

const PET_Z: f32 = 1.0;
pub const PET_SIZE: Vec2 = const_vec2!([64., 64.]);

#[derive(Component)]
pub struct Pet {}

#[derive(Bundle)]
struct PetBundle {
    pet: Pet,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl Pet {
    pub fn new() -> Self {
        Self {}
    }

    pub fn spawn(&self, location: Vec2, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let pet = Self::new();

        let texture = asset_server.load("textures/hook.png");

        let sprite_bundle = SpriteBundle {
            texture,
            transform: Transform::from_xyz(location.x, location.y, PET_Z),
            sprite: Sprite {
                custom_size: Some(PET_SIZE),
                flip_x: true,
                ..Default::default()
            },
            ..default()
        };

        let pet_bundle = PetBundle { pet, sprite_bundle };

        commands.spawn_bundle(pet_bundle);
    }
}
