use crate::game;
use bevy::prelude::{Plugin as BevyPlugin, *};

mod resources;
mod services;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(game::State::Play).with_system(setup_game))
            .add_system_set(SystemSet::on_update(game::State::Play).with_system(update_game))
            .add_system_set(SystemSet::on_exit(game::State::Play).with_system(teardown_game));
    }
}

fn setup_game() {
    println!("setup");
}

fn update_game() {
    println!("update");
}

fn teardown_game() {
    println!("teardown");
}
