use bevy::prelude::*;
// use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;

use the_ablockination::{config, game, resources, scenes};

fn main() {
    App::new()
        .add_plugin(config::Plugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(resources::Plugin)
        .add_plugin(scenes::Plugin)
        .add_state(game::State::Startup)
        .run();

    // Not for release
    // app.add_plugin(WorldInspectorPlugin::new());
}
