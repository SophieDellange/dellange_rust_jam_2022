use crate::{
    game,
    scenes::ingame::{resources::TileAtlas, services::MapGenerator},
};
use bevy::{
    math::const_vec2,
    prelude::{Plugin as BevyPlugin, *},
    render::camera::Camera2d,
};

use self::resources::{Map, TILE_SIZE};

mod resources;
mod services;

const MAP_SIZE: (u16, u16) = (32, 15); // (width, height)

const PLAYER_MOVE_SPEED: Vec2 = const_vec2!([10., 10.]); // pixels

const ENEMIES_COUNT: u8 = 16;

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
                .with_system(update_game.after(move_player))
                .with_system_set(),
        )
        .add_system_set(SystemSet::on_exit(game::State::Play).with_system(teardown_game));
    }
}

// Return the coordinates of (top left, bottom right)
//
fn camera_limits(windows: Res<Windows>) -> (Vec2, Vec2) {
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

    let (top_left, _) = camera_limits(windows);

    camera.transform = Transform::from_xyz(top_left.x, top_left.y, 999.);

    commands.spawn_bundle(camera);
}

fn generate_map_and_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_atlas = TileAtlas::new(&asset_server);
    let map_generator = MapGenerator::new(tile_atlas, MAP_SIZE.0, MAP_SIZE.1);
    let map: Map = map_generator.build_map();

    map_generator.generate_enemies(ENEMIES_COUNT, &mut commands, &asset_server);

    for (row_i, row) in map.tiles.iter().enumerate() {
        for (col_i, tile) in row.iter().enumerate() {
            // The anchor is in the center, so must readjust.
            let tile_shift = Vec2::new(TILE_SIZE.x / 2.0, -TILE_SIZE.y / 2.0);

            let tile_location =
                tile_shift + Vec2::new(col_i as f32 * TILE_SIZE.x, -(row_i as f32 * TILE_SIZE.y));

            tile.spawn(tile_location, &mut commands);
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

    let (top_left, bottom_right) = camera_limits(windows);

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

fn move_bullet(mut q_bullets: Query<&mut Transform, With<Bullet>>, windows: Res<Windows>) {
    q_bullets.iter_mut().each(| transform, bullet| {

        transform.translation.x += 
        transform.translation.y += 
        

    });
}

fn update_game() {
    //println!("update");
}

fn teardown_game() {
    //println!("teardown");
}
