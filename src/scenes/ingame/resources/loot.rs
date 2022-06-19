use bevy::{math::const_vec2, prelude::*};

const LOOT_Z: f32 = 1.0;
const LOOT_SIZE: Vec2 = const_vec2!([64., 64.]);

#[derive(Component)]
pub struct Loot {}

#[derive(Bundle)]
struct LootBundle {
    player: Loot,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl Loot {
    pub fn new() -> Self {
        Loot {}
    }

    pub fn spawn(&self, location: Vec2, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let player = Loot::new();

        let texture = asset_server.load("textures/loot_gold_coin.png");

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
            player,
            sprite_bundle,
        };

        commands.spawn_bundle(player_bundle);
    }
}
