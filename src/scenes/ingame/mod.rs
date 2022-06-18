use crate::{game, scenes::ingame::services::MapGenerator};
use bevy::{
    math::const_vec2,
    prelude::{Plugin as BevyPlugin, *},
};

use self::resources::Map;

mod resources;
mod services;

const TILE_SIZE: Vec2 = const_vec2!([30., 30.]); // pixels
const TILES_Z: f32 = 1.;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(game::State::Play).with_system(generate_map_and_tiles),
        )
        .add_system_set(SystemSet::on_update(game::State::Play).with_system(update_game))
        .add_system_set(SystemSet::on_exit(game::State::Play).with_system(teardown_game));
    }
}

fn generate_map_and_tiles(mut commands: Commands) {
    println!("> generate_map");

    let map: Map = MapGenerator::new().build_map(50, 10);

    for (row_i, row) in map.tiles.iter().enumerate() {
        for (col_i, tile) in row.iter().enumerate() {
            let tile_location = Vec2::new(col_i as f32 * TILE_SIZE.x, row_i as f32 * TILE_SIZE.y);

            println!("  > tile L:{} C:{:?}", tile_location, tile.color);

            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: tile.color,
                    custom_size: Some(TILE_SIZE),
                    ..default()
                },

                transform: Transform::from_xyz(tile_location.x, tile_location.y, TILES_Z),

                ..default()
            });
        }
    }
}

fn update_game() {
    println!("update");
}

fn teardown_game() {
    println!("teardown");
}
