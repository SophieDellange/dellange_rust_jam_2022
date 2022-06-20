use bevy::prelude::{Plugin as BevyPlugin, *};
use bevy_inspector_egui::RegisterInspectable;

mod camera_utils;
mod components;
mod constants;
mod resources;
mod services;
mod systems;

use self::{resources::BlockData, systems::*};
use crate::game;

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
                .with_system(update_game.after(move_player))
                .with_system(resources::spawn_bullets)
                .with_system(resources::move_bullets)
                .with_system(resources::check_or_bullet_collisions)
                .with_system(pet_pick_loot.after(move_pet))
                .with_system(pet_move_loot.after(move_pet)),
        )
        .add_system_set(SystemSet::on_exit(game::State::Play).with_system(teardown_game))
        .add_event::<resources::BulletCollisionEvent>()
        .register_type::<BlockData>();
    }
}
