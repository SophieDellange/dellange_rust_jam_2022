use std::time::Duration;

use bevy::{prelude::*, render::camera::Camera2d};
use bevy_kira_audio::{Audio, AudioChannel};
use rand::{thread_rng, Rng};

use crate::game;

#[allow(clippy::wildcard_imports)]
use super::{
    camera_utils::*, components::LootTransported, constants::*, resources::*, services::*,
};

// When adding new entries, remember to remove them in the cleanup() system.
//
pub fn init_resources(mut commands: Commands) {
    commands.insert_resource(EnemyBulletTimer(Timer::new(
        Duration::from_secs_f32(ENEMY_BULLET_INTERVAL),
        false,
    )))
}

pub fn spawn_camera(mut commands: Commands, windows: Res<Windows>) {
    let mut camera = OrthographicCameraBundle::new_2d();

    let (top_left, _) = camera_limits(&windows);

    camera.transform = Transform::from_xyz(top_left.x, top_left.y, 999.);

    commands.spawn_bundle(camera);
}

pub fn spawn_ui(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
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

#[allow(clippy::cast_possible_truncation)]
pub fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..ENEMIES_COUNT {
        let location = Vec2::new(
            f32::from(thread_rng().gen_range(0..(MAP_SIZE.0 * TILE_SIZE as u16))),
            -f32::from(thread_rng().gen_range(0..(MAP_SIZE.1 * TILE_SIZE as u16))),
        );

        EnemyBundle::spawn(location, &mut commands, None, &asset_server);
    }
}

#[allow(clippy::cast_possible_truncation)]
pub fn spawn_enemies_tsunami(
    mut commands: Commands,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    timer: Res<Time>,
) {
    let difficulty =
        Some(timer.seconds_since_startup() as f32 / DIFFICULTY_RAMP_UP_EVERY_NTH_SECONDS);

    let enemies_amount = thread_rng().gen_range(0..ENEMIES_COUNT);

    for _ in 0..=enemies_amount {
        let location = Vec2::new(
            f32::from(thread_rng().gen_range(0..(MAP_SIZE.0 * TILE_SIZE.abs() as u16))),
            -f32::from(thread_rng().gen_range(0..(MAP_SIZE.1 * TILE_SIZE.abs() as u16))),
        );

        EnemyBundle::spawn(location, &mut commands, difficulty, &asset_server);
    }

    if enemies_amount > 0 {
        audio.play_in_channel(
            asset_server.load(SOUND_ENEMY_GROWL),
            &AudioChannel::new(AUDIO_EFFECTS_CHANNEL.to_owned()),
        );
    }
}

pub fn spawn_loot(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..LOOT_COUNT {
        let loot_location = Vec2::new(
            thread_rng().gen_range(0..(MAP_SIZE.0 * TILE_SIZE as u16)) as f32,
            -(thread_rng().gen_range(0..(MAP_SIZE.1 * TILE_SIZE as u16)) as f32),
        );

        Loot::spawn(loot_location, &mut commands, &asset_server);
    }
}

pub fn spawn_player_and_pet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    let player_location = Vec2::new(window.width() / 5., -window.height() / 2.);

    PlayerCoreTile::spawn(player_location, &mut commands, &asset_server);

    let pet_location = player_location + Vec2::new(48., 56.);

    Pet::spawn(pet_location, &mut commands, &asset_server);
}

pub fn spawn_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scoreboard = ScoreBoard::new(&asset_server);
    commands.spawn_bundle(scoreboard);
}

pub fn move_player_tiles(
    keys: Res<Input<KeyCode>>,
    mut q_player_tiles_transform: Query<
        &mut Transform,
        Or<(With<PlayerCoreTile>, With<PlayerExtraTile>)>,
    >,
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
        player_tile_transform.translation.x += normalized_diff.x;
        player_tile_transform.translation.y += normalized_diff.y;
    }
}

pub fn move_pet(
    windows: Res<Windows>,
    q_camera: Query<&GlobalTransform, With<Camera2d>>,
    mut q_pet: Query<&mut Transform, With<Pet>>,
) {
    if let Ok(mut transform) = q_pet.get_single_mut() {
        let window = windows.get_primary().unwrap();

        if let Some(mouse_pos) = window.cursor_position() {
            let camera_translation = q_camera.single().translation.truncate();
            let pet_traslation = &mut transform.translation;

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
    };
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
        if let Ok(pet_transform) = q_pet.get_single() {
            let pet_location = pet_transform.translation.truncate();

            for (loot_entity, loot_location) in q_loot.iter() {
                let loot_location = loot_location.translation.truncate();
                let loot_distance = (pet_location - loot_location).length().abs();

                if loot_distance <= PET_PICK_LOOT_RADIUS {
                    commands.entity(loot_entity).insert(LootTransported::new());
                }
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
    if let Ok(pet_transform) = q.p0().get_single() {
        let pet_location = pet_transform.translation;

        let mut q1 = q.p1();
        let loot_transported = q1.get_single_mut();

        if let Ok(mut loot_transported) = loot_transported {
            loot_transported.translation.x = pet_location.x;
            loot_transported.translation.y = pet_location.y;
        }
    }
}
// Arbitrary; can be much smaller.
//
const EPSILON: f32 = 0.1;

pub fn pet_lock_loot(
    mut commands: Commands,
    mut q: ParamSet<(
        Query<&Transform, Or<(With<PlayerCoreTile>, With<PlayerExtraTile>)>>,
        Query<&Transform, With<LootTransported>>,
        Query<(Entity, &mut Transform), With<TileLock>>,
    )>,
    asset_server: Res<AssetServer>,
) {
    // The problem of finding the available positions is actually not as simple as one would think,
    // for several reasons (e.g. floats can't be hashed without rounding; hashing with rounding screams
    // for errors, etc.etc).
    //
    // We therefore apply a st00pid simple solution:
    //
    // - if the loot position is within a given distance from a player tile, it has at most two potential
    //   positions
    // - we collect the potential positions
    // - we filter out the occupied ones
    // - we sort them from closes to farthest
    // - we pick the closest

    let radius = Vec2::new(TILE_SIZE, TILE_SIZE).length();

    let q_loot_transform = q.p1();

    if let Ok(loot_transform) = q_loot_transform.get_single() {
        let loot_position = loot_transform.translation.truncate();

        let q_player_tiles_transform = q.p0();

        let player_tile_positions = q_player_tiles_transform
            .iter()
            .map(|transform| transform.translation.truncate())
            .collect::<Vec<_>>();

        let mut potential_positions = vec![];

        // The functionally composed version is more confusing.
        //
        for player_tile_position in &player_tile_positions {
            let distance_vec = loot_position - *player_tile_position;

            // For simplicity, we put both positions (horizontal and vertical).
            //
            if distance_vec.length() <= radius {
                potential_positions.push(Vec2::new(
                    player_tile_position.x,
                    player_tile_position.y + (TILE_SIZE * distance_vec.y.signum()),
                ));
                potential_positions.push(Vec2::new(
                    player_tile_position.x + (TILE_SIZE * distance_vec.x.signum()),
                    player_tile_position.y,
                ));
            }
        }

        let mut available_positions = potential_positions
            .into_iter()
            .filter(|potential_position| {
                player_tile_positions.iter().any(|player_tile_position| {
                    (*player_tile_position - *potential_position).length() > EPSILON
                })
            })
            .collect::<Vec<_>>();

        available_positions.sort_by(|available_pos1, available_pos2| {
            let dist1 = (loot_position - *available_pos1).length();
            let dist2 = (loot_position - *available_pos2).length();

            dist1.partial_cmp(&dist2).unwrap()
        });

        let mut q_tile_lock = q.p2();
        let tile_lock = q_tile_lock.get_single_mut();

        if let Some(best_position) = available_positions.first() {
            if let Ok((_, mut tile_lock)) = tile_lock {
                tile_lock.translation.x = best_position.x;
                tile_lock.translation.y = best_position.y;
            } else {
                TileLock::spawn(*best_position, &mut commands, &asset_server);
            }
        } else if let Ok((lock_entity, _)) = tile_lock {
            commands.entity(lock_entity).despawn();
        }
    }
}

pub fn pet_attach_loot(
    mut commands: Commands,
    q_loot_lock: Query<(Entity, &mut Transform), With<TileLock>>,
    q_loot_transported: Query<(Entity, &Loot), With<LootTransported>>,
    q_mouse_buttons: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((loot_lock_id, loot_transform)) = q_loot_lock.get_single() {
        if q_mouse_buttons.just_pressed(MouseButton::Left) {
            let (loot_transported_id, loot) = q_loot_transported.get_single().unwrap();

            PlayerExtraTile::spawn(
                &loot.loot_type,
                loot_transform.translation.truncate(),
                &mut commands,
                &asset_server,
            );

            commands.entity(loot_lock_id).despawn();
            commands.entity(loot_transported_id).despawn();
        }
    }
}

pub fn move_camera(
    q_player_transform: Query<&Transform, With<PlayerCoreTile>>,
    mut q_camera: Query<&mut GlobalTransform, With<Camera2d>>,
    windows: Res<Windows>,
) {
    if let Ok(player_transform) = q_player_transform.get_single() {
        let player_translation = player_transform.translation;
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
}

pub fn move_enemies(mut q_enemies: Query<(&mut Transform, &mut RandomMovement)>, time: Res<Time>) {
    for (mut enemy_transform, mut movement) in q_enemies.iter_mut() {
        movement.timer.tick(time.delta());

        enemy_transform.translation += movement.direction * ENEMIES_SPEED;

        if movement.timer.finished() {
            movement.renew();
        }
    }
}

pub fn gameover(
    mut commands: Commands,
    q_player_core_tile: Query<(), With<PlayerCoreTile>>,
    q_player_extra_tiles: Query<Entity, With<PlayerExtraTile>>,
    q_pet: Query<Entity, With<Pet>>,
    mut l_gameover_timer: Local<Option<Timer>>,
    mut state: ResMut<State<game::State>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    if q_player_core_tile.get_single().is_err() {
        if let Some(gameover_timer) = l_gameover_timer.as_mut() {
            gameover_timer.tick(time.delta());

            if gameover_timer.finished() {
                // Don't forget to reset, otherwise, the subsequent game over scenes will finish immediately!
                //
                *l_gameover_timer = None;
                state.set(game::State::Title).unwrap();
            }
        } else {
            // Ensure that all the other tiles are removed. This can be removed if/when the logic to
            // remove disconnected tiles is wired (as they'll be despawned by that service).
            //
            // We don't despawn the player bullets; this is intentional, but they can also be despawned
            // if it gives a better look.
            //
            for tile_id in q_player_extra_tiles.iter() {
                commands.entity(tile_id).despawn();
            }

            for pet_id in q_pet.iter() {
                commands.entity(pet_id).despawn();
            }

            *l_gameover_timer = Some(Timer::new(GAMEOVER_TIME, false));

            let style = Style {
                align_self: AlignSelf::FlexEnd,
                // I'm exhausted of trawling through docs and examples for such a simple operation as
                // centering a text.
                //
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(290.),
                    left: Val::Px(320.),
                    ..default()
                },
                ..default()
            };

            let text = Text::with_section(
                "GAME OVER",
                TextStyle {
                    font: asset_server.load(FONT_LOCATION),
                    font_size: 64.,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            );

            let text_bundle = TextBundle {
                style,
                text,
                ..default()
            };

            commands.spawn_bundle(text_bundle);
        }
    }
}

pub fn initialize_audio_channels(audio: Res<Audio>, assets: Res<AssetServer>) {
    let music_chan = AudioChannel::new(AUDIO_MUSIC_CHANNEL.to_owned());
    audio.set_volume_in_channel(DEFAULT_MUSIC_VOLUME, &music_chan);
    audio.set_volume_in_channel(
        DEFAULT_EFFECT_VOLUME,
        &AudioChannel::new(AUDIO_EFFECTS_CHANNEL.to_owned()),
    );
    audio.set_volume_in_channel(
        DEFAULT_INTERFACE_VOLUME,
        &AudioChannel::new(AUDIO_INTERFACE_CHANNEL.to_owned()),
    );

    audio.play_looped_in_channel(assets.load(MUSIC_MAIN_THEME), &music_chan);
}

pub fn cleanup(world: &mut World) {
    world.clear_entities();

    world.remove_resource::<EnemyBulletTimer>();
}
