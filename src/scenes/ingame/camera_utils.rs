use bevy::prelude::*;

#[allow(clippy::wildcard_imports)]
use super::{constants::*, resources::TILE_SIZE};

// Return the coordinates of (top left, bottom right)
//
pub fn camera_limits(windows: &Res<Windows>) -> (Vec2, Vec2) {
    let window = windows.get_primary().unwrap();

    // For simplicity shift the camera top left to (0.0).
    // DON'T FORGET THE Y SIGN!!

    let top_left = Vec2::new(window.width() / 2., -window.height() / 2.);
    let bottom_right = top_left
        + Vec2::new(
            (f32::from(MAP_SIZE.0) * TILE_SIZE) - window.width(),
            -(f32::from(MAP_SIZE.1) * TILE_SIZE) + window.height(),
        );

    (top_left, bottom_right)
}

// When the player is Within this area, the camera doesn't pan.
//
// Return the coordinates of (top left, bottom right)
//
pub fn nopan_area(windows: &Res<Windows>, camera_location: Vec2) -> (Vec2, Vec2) {
    let window = windows.get_primary().unwrap();

    // Coordinates are relative to the center of the camera.

    let top_left = Vec2::new(-window.width() * 3. / 8., window.height() / 4.);

    let bottom_right = Vec2::new(0., -window.height() / 4.);

    (camera_location + top_left, camera_location + bottom_right)
}
