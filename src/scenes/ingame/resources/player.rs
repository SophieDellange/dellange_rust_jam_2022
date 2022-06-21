use bevy::prelude::*;

// Marks a tile that belongs to the player
#[derive(Component)]
pub struct Player {}

impl Player {
    pub fn new() -> Self {
        Self {}
    }
}
