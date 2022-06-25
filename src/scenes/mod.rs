mod ingame;
mod title;

use bevy::prelude::{Plugin as BevyPlugin, *};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(title::Plugin).add_plugin(ingame::Plugin);
    }
}
