use bevy::prelude::*;

// Signals that a loot is being transported
//
#[derive(Component)]
pub struct LootTransported {}

impl LootTransported {
    pub fn new() -> Self {
        Self {}
    }
}
