use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;

use dellange_rust_jam_2022::{
    config, game,
    resources::{self, prelude::*},
    scenes,
};

fn main() {
    App::new()
        .add_plugin(config::Plugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(resources::Plugin)
        .add_plugin(scenes::Plugin)
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(Colors::DARK))
        .add_state(game::State::Startup)
        .run();
}
