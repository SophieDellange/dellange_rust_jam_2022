mod embossed;
mod simple;

pub use embossed::Embossed;
pub use simple::Simple;

use bevy::prelude::*;

const TEXT_ALIGNMENT: TextAlignment = TextAlignment {
    vertical: VerticalAlign::Center,
    horizontal: HorizontalAlign::Center,
};

const SMALL_SIZE: f32 = 14.0;
const MEDIUM_SIZE: f32 = 28.0;
const BIG_SIZE: f32 = 42.0;
