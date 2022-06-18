mod ui;

use crate::{game, resources::prelude::*};
use bevy::prelude::{Plugin as BevyPlugin, *};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(game::State::Loading).with_system(setup))
            .add_system_set(SystemSet::on_exit(game::State::Loading).with_system(cleanup));
    }
}

fn setup(mut commands: Commands, fonts: Res<Fonts>) {
    ui::spawn(&mut commands, &fonts);
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<ui::ScopedMarker>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
