mod colors;
mod fonts;
pub mod prelude;

use crate::game;
use bevy::prelude::{Plugin as BevyPlugin, *};
use prelude::*;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(game::State::Startup).with_system(startup))
            .add_system_set(SystemSet::on_update(game::State::Loading).with_system(check_loading));
    }
}

fn startup(
    mut commands: Commands,
    mut state: ResMut<State<game::State>>,
    asset_server: Res<AssetServer>,
) {
    let fonts = Fonts::load(&asset_server);
    commands.insert_resource(fonts);

    state.set(game::State::Loading).unwrap();
}

fn check_loading(
    mut state: ResMut<State<game::State>>,
    asset_server: Res<AssetServer>,
    fonts: Res<Fonts>,
) {
    let all_loaded = fonts.all_loaded(&asset_server);

    if all_loaded {
        state.set(game::State::Title).unwrap();
    }
}
