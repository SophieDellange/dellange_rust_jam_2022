use crate::{
    game,
    scenes::ingame::{resources::TileAtlas, services::MapGenerator},
};
use bevy::{
    math::const_vec2,
    prelude::{Plugin as BevyPlugin, *},
    render::camera::Camera2d,
};

use self::resources::Map;

mod resources;
mod services;

const TILE_SIZE: Vec2 = const_vec2!([64., 64.]); // pixels
const TILES_Z: f32 = 0.;

const MAP_SIZE: (u16, u16) = (32, 15); // (width, height)

const PLAYER_MOVE_SPEED: Vec2 = const_vec2!([10., 10.]); // pixels

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(game::State::Play)
                .with_system(spawn_camera)
                .with_system(generate_map_and_tiles.after(spawn_camera)),
        )
        .add_system_set(
            SystemSet::on_update(game::State::Play)
                .with_system(move_player)
                .with_system(update_game.after(move_player)),
        )
        .add_system_set(SystemSet::on_exit(game::State::Play).with_system(teardown_game));
    }
}

// Return the coordinates of (top left, bottom right)
//
fn map_limits(windows: Res<Windows>) -> (Vec2, Vec2) {
    let window = windows.get_primary().unwrap();

    // For simplicity shift the camera top left to (0.0).
    // DON'T FORGET THE Y SIGN!!

    let top_left = Vec2::new(window.width() / 2., -window.height() / 2.);
    let bottom_right = top_left
        + Vec2::new(
            (MAP_SIZE.0 as f32 * TILE_SIZE.x) - window.width(),
            -(MAP_SIZE.1 as f32 * TILE_SIZE.y) + window.height(),
        );

    (top_left, bottom_right)
}

fn spawn_camera(mut commands: Commands, windows: Res<Windows>) {
    let mut camera = OrthographicCameraBundle::new_2d();

    let (top_left, _) = map_limits(windows);

    camera.transform = Transform::from_xyz(top_left.x, top_left.y, 999.);

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

fn move_player(
    keys: Res<Input<KeyCode>>,
    mut q_camera: Query<&mut GlobalTransform, With<Camera2d>>,
    windows: Res<Windows>,
) {
    let mut camera_transform = q_camera.single_mut();

    let (camera_x, camera_y) = (
        camera_transform.translation.x,
        camera_transform.translation.y,
    );

    let (top_left, bottom_right) = map_limits(windows);

    let (mut x_diff, mut y_diff) = (0., 0.);

    if keys.pressed(KeyCode::W) {
        y_diff = PLAYER_MOVE_SPEED.y;
    } else if keys.pressed(KeyCode::A) {
        x_diff = -PLAYER_MOVE_SPEED.x;
    } else if keys.pressed(KeyCode::S) {
        y_diff = -PLAYER_MOVE_SPEED.y;
    } else if keys.pressed(KeyCode::D) {
        x_diff = PLAYER_MOVE_SPEED.x;
    }

    let new_camera_x = (camera_x + x_diff).clamp(top_left.x, bottom_right.x);
    let new_camera_y = (camera_y + y_diff).clamp(bottom_right.y, top_left.y);

    camera_transform.translation.x = new_camera_x;
    camera_transform.translation.y = new_camera_y;
}

fn update_game() {
    println!("update");
}

fn teardown_game() {
    println!("teardown");
}
