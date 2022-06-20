use bevy::{prelude::*, render::camera::Camera2d};
use rand::{thread_rng, Rng};

use super::{
    camera_utils::*, components::LootTransported, constants::*, resources::*, services::*,
};

pub fn spawn_camera(mut commands: Commands, windows: Res<Windows>) {
    let mut camera = OrthographicCameraBundle::new_2d();

    let (top_left, _) = camera_limits(&windows);

    camera.transform = Transform::from_xyz(top_left.x, top_left.y, 999.);

    commands.spawn_bundle(camera);
}

pub fn generate_map_and_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_atlas = TileAtlas::new(&asset_server);
    let map_generator = MapGenerator::new(tile_atlas, MAP_SIZE.0, MAP_SIZE.1);
    let map: Map = map_generator.build_map();

    for (row_i, row) in map.tiles.iter().enumerate() {
        for (col_i, tile) in row.iter().enumerate() {
            // The anchor is in the center, so must readjust.
            let tile_shift = Vec2::new(TILE_SIZE / 2.0, -TILE_SIZE / 2.0);

            let tile_location =
                tile_shift + Vec2::new(col_i as f32 * TILE_SIZE, -(row_i as f32 * TILE_SIZE));

            tile.spawn(tile_location, &mut commands);
        }
    }
}

pub fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..ENEMIES_COUNT {
        let location = Vec2::new(
            thread_rng().gen_range(0..(MAP_SIZE.0 * TILE_SIZE as u16)) as f32,
            -(thread_rng().gen_range(0..(MAP_SIZE.1 * TILE_SIZE as u16)) as f32),
        );

        Enemy::new(&asset_server).spawn(location, &mut commands);
    }
}

pub fn spawn_loot(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..LOOT_COUNT {
        let loot_location = Vec2::new(
            thread_rng().gen_range(0..(MAP_SIZE.0 * TILE_SIZE as u16)) as f32,
            -(thread_rng().gen_range(0..(MAP_SIZE.1 * TILE_SIZE as u16)) as f32),
        );

        Loot::new().spawn(loot_location, &mut commands, &asset_server);
    }
}

pub fn spawn_player_and_pet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    let player_location = Vec2::new(window.width() / 5., -window.height() / 2.);

    PlayerTile::new().spawn(player_location, &mut commands, &asset_server);

    let pet_location = player_location + Vec2::new(48., 56.);

    Pet::new().spawn(pet_location, &mut commands, &asset_server);
}

pub fn move_player(
    keys: Res<Input<KeyCode>>,
    mut q_player_tiles_transform: Query<&mut Transform, With<PlayerTile>>,
) {
    let (mut x_diff, mut y_diff) = (0., 0.);

    if keys.pressed(KeyCode::W) {
        y_diff = 1.;
    }
    if keys.pressed(KeyCode::A) {
        x_diff = -1.;
    }
    if keys.pressed(KeyCode::S) {
        y_diff = -1.;
    }
    if keys.pressed(KeyCode::D) {
        x_diff = 1.;
    }

    let normalized_diff = Vec2::new(x_diff, y_diff).normalize_or_zero() * PLAYER_MOVE_SPEED;

    for mut player_tile_transform in q_player_tiles_transform.iter_mut() {
        player_tile_transform.translation.x =
            player_tile_transform.translation.x + normalized_diff.x;
        player_tile_transform.translation.y =
            player_tile_transform.translation.y + normalized_diff.y;
    }
}

pub fn move_pet(
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

pub fn pet_pick_loot(
    mut commands: Commands,
    q_loot_transported: Query<&LootTransported>,
    q_loot: Query<(Entity, &Transform), With<Loot>>,
    q_mouse_buttons: Res<Input<MouseButton>>,
    q_pet: Query<&Transform, With<Pet>>,
) {
    let any_loot_transported = q_loot_transported.get_single().is_ok();

    if !any_loot_transported && q_mouse_buttons.just_pressed(MouseButton::Left) {
        let pet_location = q_pet.single().translation.truncate();

        for (loot_entity, loot_location) in q_loot.iter() {
            let loot_location = loot_location.translation.truncate();
            let loot_distance = (pet_location - loot_location).length().abs();

            if loot_distance <= PET_PICK_LOOT_RADIUS {
                commands.entity(loot_entity).insert(LootTransported::new());
            }
        }
    }
}

pub fn pet_move_loot(
    mut q: ParamSet<(
        Query<&Transform, With<Pet>>,
        Query<&mut Transform, With<LootTransported>>,
    )>,
) {
    let pet_location = q.p0().single().translation;

    let mut q1 = q.p1();
    let loot_transported = q1.get_single_mut();

    if let Ok(mut loot_transported) = loot_transported {
        loot_transported.translation.x = pet_location.x;
        loot_transported.translation.y = pet_location.y;
    }
}

pub fn move_camera(
    q_player_transform: Query<&Transform, With<PlayerTile>>,
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

pub fn update_game() {
    // println!("update");
}

pub fn teardown_game() {
    // println!("teardown");
}
