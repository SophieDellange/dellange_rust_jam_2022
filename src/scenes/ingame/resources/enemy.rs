use bevy::prelude::*;

// Marks something that belongs to an enemy
#[derive(Component)]
pub struct Enemy {}

impl Enemy {
    pub fn new() -> Self {
        Self {}
    }
}
