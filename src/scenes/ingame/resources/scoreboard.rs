// This is due to #[derive(Bundle)]  issue https://github.com/bevyengine/bevy/issues/4601
#![allow(clippy::forget_non_drop)]

use bevy::prelude::*;

#[derive(Component)]
pub struct Score(pub u32);

#[derive(Bundle)]
pub struct ScoreBoard {
    pub score: Score,
    #[bundle]
    pub text_bundle: TextBundle,
    // pub color_text: ColorText,
}

const TEXT_POSITION_TOP: f32 = 100.;
const TEXT_POSITION_RIGHT: f32 = 50.;
const FONT_LOCATION: &str = "fonts/dogica/dogicapixel.ttf";
const FONT_SIZE: f32 = 64.;
const FONT_COLOR: Color = Color::WHITE;

impl ScoreBoard {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        let score = Score(0);

        let style = Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                right: Val::Px(TEXT_POSITION_TOP),
                top: Val::Px(TEXT_POSITION_RIGHT),
                ..default()
            },
            ..default()
        };

        let text = Text::with_section(
            format!("{}", score.0),
            TextStyle {
                font: asset_server.load(FONT_LOCATION),
                font_size: FONT_SIZE,
                color: FONT_COLOR,
            },
            // Note: You can use `Default::default()` in place of the `TextAlignment`
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..default()
            },
        );

        let text_bundle = TextBundle {
            style,
            text,
            ..default()
        };

        Self { score, text_bundle }
    }
}
