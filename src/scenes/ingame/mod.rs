use bevy::{
    core::FixedTimestep,
    prelude::{Plugin as BevyPlugin, *},
};

mod camera_utils;
mod components;
mod constants;
mod resources;
mod services;
mod systems;

#[allow(clippy::wildcard_imports)]
use self::{resources::BlockData, systems::*};
use crate::game;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(game::State::Play)
                .with_system(init_resources)
                .with_system(spawn_camera)
                .with_system(spawn_ui)
                .with_system(generate_map_and_tiles)
                .with_system(spawn_enemies)
                .with_system(spawn_loot)
                .with_system(spawn_player_and_pet)
                .with_system(initialize_audio_channels)
                .with_system(spawn_scoreboard),
        )
        .add_system_set(
            SystemSet::on_update(game::State::Play)
                .with_system(move_player_tiles)
                .with_system(move_pet)
                .with_system(move_camera.after(move_player_tiles))
                .with_system(move_enemies)
                .with_system(resources::spawn_player_bullets)
                .with_system(resources::spawn_enemy_bullets)
                .with_system(resources::move_bullets)
                .with_system(resources::check_or_bullet_collisions)
                .with_system(resources::bullet_hits.after(resources::check_or_bullet_collisions))
                .with_system(resources::update_scoreboard.after(resources::bullet_hits))
                .with_system(pet_pick_loot.after(move_pet))
                .with_system(pet_move_loot.after(move_pet))
                .with_system(pet_lock_loot.after(pet_move_loot))
                .with_system(pet_attach_loot.after(pet_move_loot))
                .with_system(resources::health_based_status.after(resources::bullet_hits))
                .with_system(gameover.after(resources::bullet_hits)),
        )
        .add_system_set(
            SystemSet::on_update(game::State::Play)
                .with_run_criteria(FixedTimestep::step(8.0))
                .with_system(spawn_enemies_tsunami),
        )
        .add_system_set(
            SystemSet::on_exit(game::State::Play).with_system(cleanup.exclusive_system()),
        )
        .add_event::<resources::BulletCollisionEvent>()
        .register_type::<BlockData>();
    }
}
