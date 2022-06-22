use bevy::prelude::*;

// Marks something that belongs to the player
#[derive(Component)]
pub struct Player {}

impl Player {
    pub fn new() -> Self {
        Self {}
    }
}
