use bevy::prelude::*;

use crate::resources::prelude::Colors;

use super::text::Simple;

pub enum Kind {
    Play,
    Quit,
}

#[derive(Component)]
pub struct Marker {
    kind: Kind,
}

impl Marker {
    #[must_use]
    pub fn new(kind: Kind) -> Marker {
        Marker { kind }
    }

    #[must_use]
    pub fn play() -> Marker {
        Marker::new(Kind::Play)
    }

    #[must_use]
    pub fn quit() -> Marker {
        Marker::new(Kind::Quit)
    }

    #[must_use]
    pub fn kind(&self) -> &Kind {
        &self.kind
    }
}

pub struct Action {
    bundle: ButtonBundle,
    child: Simple,
}

impl Default for Action {
    fn default() -> Action {
        let style = Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Percent(100.0), Val::Px(50.0)),
            ..Default::default()
        };
        let child = Simple::default();

        Action {
            bundle: ButtonBundle {
                style,
                color: Colors::PRIMARY.into(),
                ..Default::default()
            },
            child,
        }
    }
}

impl Action {
    pub fn new<S: Into<String>>(value: S, font: &Handle<Font>, size: Size<Val>) -> Action {
        let mut child = Simple::medium(value, font);
        child.color(Colors::DARK);

        let mut button = Action::default();
        button.bundle.style.size = size;
        button.child = child;

        button
    }

    pub fn spawn(self, parent: &mut ChildBuilder, marker: Marker) {
        parent
            .spawn_bundle(self.bundle)
            .with_children(|parent| self.child.spawn(parent))
            .insert(marker);
    }
}
