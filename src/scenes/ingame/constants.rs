pub const MAP_SIZE: (u16, u16) = (32, 15); // (width, height)

pub const ENEMIES_COUNT: u8 = 16;
pub const LOOT_COUNT: u8 = 4;

pub const PLAYER_MOVE_SPEED: f32 = 7.5;
pub const PET_MOVE_SPEED: f32 = 20.0;

// Distance from the center of the loot, for the pet to pick the loot.
//
pub const PET_PICK_LOOT_RADIUS: f32 = 10.0;

pub const BASIC_BULLET_DAMAGE: u8 = 5;

pub const HIT_AUDIO: &'static str = "audio/SFX/SFX_Enemy_Hit.wav";
