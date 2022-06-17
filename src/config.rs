use bevy::prelude::{Plugin as BevyPlugin, *};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "Dellarange ~ Rust Jam".to_string(),
            ..Default::default()
        });
    }
}
