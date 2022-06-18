use bevy::prelude::*;

use crate::resources::prelude::Colors;

use super::{BIG_SIZE, MEDIUM_SIZE, SMALL_SIZE, TEXT_ALIGNMENT};

pub struct Simple {
    bundle: TextBundle,
}

impl Default for Simple {
    fn default() -> Simple {
        let style = TextStyle {
            font_size: MEDIUM_SIZE,
            color: Colors::LIGHT,
            font: Handle::default(),
        };

        Simple {
            bundle: TextBundle {
                text: Text::with_section("", style, TEXT_ALIGNMENT),
                ..Default::default()
            },
        }
    }
}

impl Simple {
    pub fn small<S: Into<String>>(value: S, font: &Handle<Font>) -> Simple {
        let mut simple = Simple::default();
        let section = &mut simple.bundle.text.sections[0];

        section.value = value.into();
        section.style.font = font.clone();
        section.style.font_size = SMALL_SIZE;

        simple
    }

    pub fn medium<S: Into<String>>(value: S, font: &Handle<Font>) -> Simple {
        let mut simple = Simple::default();
        let section = &mut simple.bundle.text.sections[0];

        section.value = value.into();
        section.style.font = font.clone();

        simple
    }

    pub fn big<S: Into<String>>(value: S, font: &Handle<Font>) -> Simple {
        let mut simple = Simple::default();
        let section = &mut simple.bundle.text.sections[0];

        section.value = value.into();
        section.style.font = font.clone();
        section.style.font_size = BIG_SIZE;

        simple
    }

    pub fn color(&mut self, color: Color) -> &mut Simple {
        self.bundle.text.sections[0].style.color = color;
        self
    }

    pub fn spawn(self, parent: &mut ChildBuilder) {
        parent.spawn_bundle(self.bundle);
    }
}
