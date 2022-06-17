use bevy::prelude::*;

use crate::{
    resources::prelude::*,
    ui::{Action, ActionMarker, EmbossedText, Housing, Overlay, SimpleText},
};


fn main_menu_texts(font: &Handle<Font>) -> (EmbossedText, SimpleText, SimpleText) {
    (
        EmbossedText::big("Rusty Jam\n\nDellange", font),
        SimpleText::small(
            "Sophie Dellange ÷ 64kramsystem ÷ Mechanought ÷ Joshi Spawnbrood",
            font,
        ),
        SimpleText::small("\nTemplate by septum | https://septum.io", font),
    )
}


#[derive(Component)]
pub struct ScopedMarker;

fn spawn_camera(commands: &mut Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(ScopedMarker);
}

///
/// The main title ui has two different options, depending on which target.
///  this one is the main menu for desktop game.

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn(commands: &mut Commands, fonts: &Fonts) {
    let font = &fonts.dogica;
    let button_size = Size::new(Val::Px(400.0), Val::Px(45.0));

    let overlay = Overlay::new();
    let top = Housing::percent(100.0, 50.0);
    let bottom = Housing::percent(100.0, 50.0);
    let mut actions = Housing::percent(100.0, 90.0);
    let authors_space = Housing::percent(100.0, 15.0);
    let footer = Housing::percent(100.0, 20.0);

    let (title, authors, notice) = main_menu_texts(font);

    let play = Action::new("Play", font, button_size);
    let quit = Action::new("Quit", font, button_size);

    actions
        .justify_content(JustifyContent::SpaceEvenly)
        .align_items(AlignItems::Center);

    overlay.spawn(
        commands,
        |parent| {
            top.spawn(parent, |parent| {
                title.spawn(parent);
            });
            bottom.spawn(parent, |parent| {
                actions.spawn(parent, |parent| {
                    play.spawn(parent, ActionMarker::play());
                    quit.spawn(parent, ActionMarker::quit());
                });
                authors_space.spawn(parent, |parent| {
                    authors.spawn(parent);
                });

                footer.spawn(parent, |parent| {
                    notice.spawn(parent);
                });
            });
        },
        ScopedMarker,
    );

    spawn_camera(commands);
}

/// This other one is the main menu for wasm, where buttons like "Quit" has no sense

#[cfg(target_arch = "wasm32")]
pub fn spawn(commands: &mut Commands, fonts: &Fonts) {
    let font = &fonts.dogica;
    let button_size = Size::new(Val::Px(400.0), Val::Px(45.0));

    let overlay = Overlay::new();
    let top = Housing::percent(100.0, 50.0);
    let bottom = Housing::percent(100.0, 50.0);
    let mut actions = Housing::percent(100.0, 90.0);
    let authors_space = Housing::percent(100.0, 15.0);
    let footer = Housing::percent(100.0, 20.0);

    let (title, authors, notice) = main_menu_texts(font);

    actions
        .justify_content(JustifyContent::SpaceEvenly)
        .align_items(AlignItems::Center);

    let play = Action::new("Play", font, button_size);

    overlay.spawn(
        commands,
        |parent| {
            top.spawn(parent, |parent| {
                title.spawn(parent);
            });
            bottom.spawn(parent, |parent| {
                actions.spawn(parent, |parent| {
                    play.spawn(parent, ActionMarker::play());
                });
                authors_space.spawn(parent, |parent| {
                    authors.spawn(parent);
                });

                footer.spawn(parent, |parent| {
                    notice.spawn(parent);
                });
            });
        },
        ScopedMarker,
    );

    spawn_camera(commands);
}
