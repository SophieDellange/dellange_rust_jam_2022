use bevy::{math::const_vec2, prelude::*};

use super::LootType;

const LOOT_Z: f32 = 1.0;
const LOOT_SIZE: Vec2 = const_vec2!([64., 64.]);

#[derive(Component)]
pub struct Loot {
    pub loot_type: LootType,
}

#[derive(Bundle)]
struct LootBundle {
    loot: Loot,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl Loot {
    pub fn new(loot_type: LootType) -> Self {
        Loot { loot_type }
    }

    pub fn random() -> Self {
        let loot_type = LootType::random();

        Self::new(loot_type)
    }

    pub fn spawn(&self, location: Vec2, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let loot = Loot::random();

        let texture = loot.loot_type.loot_texture(&asset_server);

        let sprite_bundle = SpriteBundle {
            texture: texture,
            transform: Transform::from_xyz(location.x, location.y, LOOT_Z),
            sprite: Sprite {
                custom_size: Some(LOOT_SIZE),
                ..Default::default()
            },
            ..default()
        };

        let player_bundle = LootBundle {
            loot,
            sprite_bundle,
        };

        commands.spawn_bundle(player_bundle);
    }
}
