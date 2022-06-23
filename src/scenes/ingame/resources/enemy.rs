use bevy::prelude::*;

pub const ENEMY_KILLED_POINTS: u32 = 10;

// Marks something that belongs to an enemy
#[derive(Component)]
pub struct Enemy {}

impl Enemy {
    pub fn new() -> Self {
        Self {}
    }
}
