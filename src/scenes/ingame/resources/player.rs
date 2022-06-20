use bevy::utils::Duration;
use bevy::{math::const_vec2, prelude::*};

use super::{BlockData, Collider};

const PLAYER_Z: f32 = 1.0;
const PLAYER_SIZE: Vec2 = const_vec2!([64., 64.]);

// Marks a tile that belongs to the player
#[derive(Component)]
pub struct Player {}

impl Player {
    pub fn new() -> Self {
        Self {}
    }
}
