use bevy::prelude::*;

use crate::resources::prelude::Colors;

use super::{BIG_SIZE, MEDIUM_SIZE, TEXT_ALIGNMENT};

pub struct Embossed {
    foreground: TextBundle,
    background: TextBundle,
}

impl Default for Embossed {
    fn default() -> Embossed {
        let relief = 2.0;
        let style = TextStyle {
            font_size: MEDIUM_SIZE,
            color: Colors::PRIMARY,
            font: Handle::default(),
        };
        let foreground = TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            text: Text::with_section("", style.clone(), TEXT_ALIGNMENT),
            ..Default::default()
        };
        let background = TextBundle {
            style: Style {
                position: Rect {
                    top: Val::Px(relief),
                    left: Val::Px(relief),
                    ..Default::default()
                },
                position_type: PositionType::Relative,
                ..Default::default()
            },
            text: Text::with_section(
                "",
                TextStyle {
                    color: Colors::DARK,
                    ..style
                },
                TEXT_ALIGNMENT,
            ),
            ..Default::default()
        };

        Embossed {
            foreground,
            background,
        }
    }
}

impl Embossed {
    pub fn big<S: Into<String>>(value: S, font: &Handle<Font>) -> Embossed {
        let string = value.into();
        let mut embossed = Embossed::default();
        let foreground_section = &mut embossed.foreground.text.sections[0];
        let background_section = &mut embossed.background.text.sections[0];

        foreground_section.value = string.clone();
        foreground_section.style.font = font.clone();
        foreground_section.style.font_size = BIG_SIZE;

        background_section.value = string;
        background_section.style.font = font.clone();
        background_section.style.font_size = BIG_SIZE;

        embossed.background.style.position.top = Val::Px(4.0);
        embossed.background.style.position.left = Val::Px(4.0);

        embossed
    }

    pub fn spawn(self, parent: &mut ChildBuilder) {
        parent.spawn_bundle(self.background);
        parent.spawn_bundle(self.foreground);
    }
}
