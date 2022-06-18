use bevy::prelude::*;

use crate::{
    resources::prelude::*,
    ui::{Overlay, SimpleText},
};

#[derive(Component)]
pub struct ScopedMarker;

fn spawn_camera(commands: &mut Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(ScopedMarker);
}

pub fn spawn(commands: &mut Commands, fonts: &Fonts) {
    let font = &fonts.dogica;
    let overlay = Overlay::new();
    let mut loading_text = SimpleText::big("Loading...", font);

    loading_text.color(Colors::PRIMARY);

    overlay.spawn(
        commands,
        |parent| {
            loading_text.spawn(parent);
        },
        ScopedMarker,
    );

    spawn_camera(commands);
}
