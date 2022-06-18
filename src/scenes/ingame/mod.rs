use crate::{
    game,
    scenes::ingame::{resources::TileAtlas, services::MapGenerator},
};
use bevy::{
    math::const_vec2,
    prelude::{Plugin as BevyPlugin, *},
};

use self::resources::Map;

mod resources;
mod services;

const TILE_SIZE: Vec2 = const_vec2!([64., 64.]); // pixels
const TILES_Z: f32 = 0.;

const MAP_SIZE: (u16, u16) = (64, 12); // (width, height)

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(game::State::Play)
                .with_system(spawn_camera)
                .with_system(generate_map_and_tiles.after(spawn_camera)),
        )
        .add_system_set(SystemSet::on_update(game::State::Play).with_system(update_game))
        .add_system_set(SystemSet::on_exit(game::State::Play).with_system(teardown_game));
    }
}

fn spawn_camera(mut commands: Commands, windows: Res<Windows>) {
    let mut camera = OrthographicCameraBundle::new_2d();

    // For simplicity shift the camera top left to (0.0).

    let window = windows.get_primary().unwrap();
    camera.transform = Transform::from_xyz(window.width() / 2., -window.height() / 2., 999.);

    commands.spawn_bundle(camera);
}

fn generate_map_and_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_atlas = TileAtlas::new(asset_server);
    let map: Map = MapGenerator::new(tile_atlas).build_map(MAP_SIZE.0, MAP_SIZE.1);

    for (row_i, row) in map.tiles.iter().enumerate() {
        for (col_i, tile) in row.iter().enumerate() {
            // The anchor is in the center, so must readjust.
            let tile_shift = Vec2::new(TILE_SIZE.x / 2.0, -TILE_SIZE.y / 2.0);

            let tile_location =
                tile_shift + Vec2::new(col_i as f32 * TILE_SIZE.x, -(row_i as f32 * TILE_SIZE.y));

            commands.spawn_bundle(SpriteBundle {
                texture: tile.texture(),
                transform: Transform::from_xyz(tile_location.x, tile_location.y, TILES_Z),
                sprite: Sprite {
                    custom_size: Some(TILE_SIZE),
                    ..Default::default()
                },
                ..default()
            });
        }
    }
}

fn update_game() {
    // println!("update");
}

fn teardown_game() {
    println!("teardown");
}
