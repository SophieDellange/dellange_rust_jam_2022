mod ui;

use bevy::{
    app::AppExit,
    prelude::{Input, Plugin as BevyPlugin, *},
};

use crate::{
    game,
    resources::prelude::*,
    ui::{ActionKind, ActionMarker},
};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(game::State::Title).with_system(setup))
            .add_system_set(
                SystemSet::on_update(game::State::Title).with_system(buttons_interactions),
            )
            .add_system_set(SystemSet::on_exit(game::State::Title).with_system(cleanup));
    }
}

fn setup(mut commands: Commands, fonts: Res<Fonts>) {
    ui::spawn(&mut commands, &fonts);
}

fn buttons_interactions(
    mut exit_event: EventWriter<AppExit>,
    mut mouse_button_input: ResMut<Input<MouseButton>>,
    mut query: Query<
        (&ActionMarker, &Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (action, interaction, mut color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                // workaround for input persistence between states
                // see: https://github.com/bevyengine/bevy/issues/1700#issuecomment-886999222
                mouse_button_input.reset(MouseButton::Left);

                match action.kind() {
                    ActionKind::Play => {
                        todo!();
                    }
                    ActionKind::Quit => {
                        exit_event.send(AppExit);
                    }
                };

                *color = Colors::DARK.into();
            }
            Interaction::Hovered => {
                *color = Colors::LIGHT.into();
            }
            Interaction::None => {
                *color = Colors::PRIMARY.into();
            }
        }
    }
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<ui::ScopedMarker>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
