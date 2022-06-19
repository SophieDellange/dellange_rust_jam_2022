use crate::{
    game,
    scenes::ingame::{resources::TileAtlas, services::MapGenerator},
};
use bevy::{
    prelude::{Plugin as BevyPlugin, *},
    render::camera::Camera2d,
};
use rand::{thread_rng, Rng};

use self::resources::{Enemy, Loot, Map, Pet, Player, TILE_SIZE};

mod resources;
mod services;

const MAP_SIZE: (u16, u16) = (32, 15); // (width, height)

const ENEMIES_COUNT: u8 = 16;
const LOOT_COUNT: u8 = 16;

const PLAYER_MOVE_SPEED: f32 = 7.5;
const PET_MOVE_SPEED: f32 = 20.0;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(game::State::Play)
                .with_system(spawn_camera)
                .with_system(generate_map_and_tiles)
                .with_system(spawn_enemies)
                .with_system(spawn_loot)
                .with_system(spawn_player_and_pet),
        )
        .add_system_set(
            SystemSet::on_update(game::State::Play)
                .with_system(move_player)
                .with_system(move_pet)
                .with_system(move_camera.after(move_player))
                .with_system(update_game.after(move_player)),
        )
        .add_system_set(SystemSet::on_exit(game::State::Play).with_system(teardown_game));
    }
}

// Return the coordinates of (top left, bottom right)
//
fn camera_limits(windows: &Res<Windows>) -> (Vec2, Vec2) {
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

// When the player is Within this area, the camera doesn't pan.
//
// Return the coordinates of (top left, bottom right)
//
fn nopan_area(windows: &Res<Windows>, camera_location: Vec2) -> (Vec2, Vec2) {
    let window = windows.get_primary().unwrap();

    // Coordinates are relative to the center of the camera.

    let top_left = Vec2::new(-window.width() * 3. / 8., window.height() / 4.);

    let bottom_right = Vec2::new(0., -window.height() / 4.);

    (camera_location + top_left, camera_location + bottom_right)
}

fn spawn_camera(mut commands: Commands, windows: Res<Windows>) {
    let mut camera = OrthographicCameraBundle::new_2d();

    let (top_left, _) = camera_limits(&windows);

    camera.transform = Transform::from_xyz(top_left.x, top_left.y, 999.);

    commands.spawn_bundle(camera);
}

fn generate_map_and_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_atlas = TileAtlas::new(&asset_server);
    let map_generator = MapGenerator::new(tile_atlas, MAP_SIZE.0, MAP_SIZE.1);
    let map: Map = map_generator.build_map();

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

fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..ENEMIES_COUNT {
        let location = Vec2::new(
            thread_rng().gen_range(0..(MAP_SIZE.0 * TILE_SIZE.x as u16)) as f32,
            -(thread_rng().gen_range(0..(MAP_SIZE.1 * TILE_SIZE.x as u16)) as f32),
        );

        Enemy::new(&asset_server).spawn(location, &mut commands);
    }
}

fn spawn_loot(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..LOOT_COUNT {
        let loot_location = Vec2::new(
            thread_rng().gen_range(0..(MAP_SIZE.0 * TILE_SIZE.x as u16)) as f32,
            -(thread_rng().gen_range(0..(MAP_SIZE.1 * TILE_SIZE.x as u16)) as f32),
        );

        Loot::new().spawn(loot_location, &mut commands, &asset_server);
    }
}

fn spawn_player_and_pet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    let player_location = Vec2::new(window.width() / 5., -window.height() / 2.);

    Player::new().spawn(player_location, &mut commands, &asset_server);

    let pet_location = player_location + Vec2::new(48., 56.);

    Pet::new().spawn(pet_location, &mut commands, &asset_server);
}

fn move_player(
    keys: Res<Input<KeyCode>>,
    mut q_player_transform: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = q_player_transform.single_mut();
    let (mut x_diff, mut y_diff) = (0., 0.);

    if keys.pressed(KeyCode::W) {
        y_diff = PLAYER_MOVE_SPEED;
    } else if keys.pressed(KeyCode::A) {
        x_diff = -PLAYER_MOVE_SPEED;
    } else if keys.pressed(KeyCode::S) {
        y_diff = -PLAYER_MOVE_SPEED;
    } else if keys.pressed(KeyCode::D) {
        x_diff = PLAYER_MOVE_SPEED;
    }

    player_transform.translation.x = player_transform.translation.x + x_diff;
    player_transform.translation.y = player_transform.translation.y + y_diff;
}

fn move_pet(
    windows: Res<Windows>,
    q_camera: Query<&GlobalTransform, With<Camera2d>>,
    mut q_pet: Query<&mut Transform, With<Pet>>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(mouse_pos) = window.cursor_position() {
        let camera_translation = q_camera.single().translation.truncate();
        let pet_traslation = &mut q_pet.single_mut().translation;

        let pet_target = Vec2::new(
            camera_translation.x - window.width() / 2. + mouse_pos.x,
            camera_translation.y - window.height() / 2. + mouse_pos.y,
        );

        let target_distance = pet_target - pet_traslation.truncate();

        if target_distance.length().abs() < PET_MOVE_SPEED {
            pet_traslation.x = pet_target.x;
            pet_traslation.y = pet_target.y;
        } else {
            let pet_move_norm = (pet_target - pet_traslation.truncate()).normalize();
            let pet_move = pet_move_norm * PET_MOVE_SPEED;

            pet_traslation.x += pet_move.x;
            pet_traslation.y += pet_move.y;
        }
    }
}

fn move_camera(
    q_player_transform: Query<&Transform, With<Player>>,
    mut q_camera: Query<&mut GlobalTransform, With<Camera2d>>,
    windows: Res<Windows>,
) {
    let player_translation = q_player_transform.single().translation;
    let camera_translation = &mut q_camera.single_mut().translation;

    let (nopan_area_top_left, nopan_area_bottom_right) =
        nopan_area(&windows, camera_translation.truncate());
    let (camera_limit_top_left, camera_limit_bottom_right) = camera_limits(&windows);

    if player_translation.x < nopan_area_top_left.x {
        camera_translation.x = (camera_translation.x + player_translation.x
            - nopan_area_top_left.x)
            .max(camera_limit_top_left.x);
    } else if player_translation.x > nopan_area_bottom_right.x {
        camera_translation.x = (camera_translation.x + player_translation.x
            - nopan_area_bottom_right.x)
            .min(camera_limit_bottom_right.x);
    }

    if player_translation.y > nopan_area_top_left.y {
        camera_translation.y = (camera_translation.y + player_translation.y
            - nopan_area_top_left.y)
            .min(camera_limit_top_left.y);
    } else if player_translation.y < nopan_area_bottom_right.y {
        camera_translation.y = (camera_translation.y + player_translation.y
            - nopan_area_bottom_right.y)
            .max(camera_limit_bottom_right.y);
    }
}

fn update_game() {
    // println!("update");
}

fn teardown_game() {
    // println!("teardown");
}
