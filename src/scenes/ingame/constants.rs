


pub const MAP_SIZE: (u16, u16) = (32, 15); // (width, height)

pub const ENEMIES_COUNT: u8 = 16;
pub const LOOT_COUNT: u8 = 4;

pub const PLAYER_MOVE_SPEED: f32 = 7.5;
pub const PET_MOVE_SPEED: f32 = 20.0;

// Distance from the center of the loot, for the pet to pick the loot.
//
pub const PET_PICK_LOOT_RADIUS: f32 = 10.0;
pub const BASIC_BULLET_DAMAGE: u8 = 5;

pub const SOUND_HIT_ENEMY: & str = "audio/SFX/SFX_Enemy_Hit.wav";
pub const SOUND_ENEMY_GROWL: & str = "audio/SFX/SFX_Enemy_Growl.wav";

pub const SOUND_ENEMY_DEATH: & str = "audio/SFX/SFX_Enemy_Death.wav";
pub const SOUND_MOVE_DOWN: & str = "audio/SFX/SFX_Move_Down.wav";
pub const SOUND_MOVE_UP: & str = "audio/SFX/SFX_Move_Up.wav";
pub const SOUND_MOVE_LEFTRIGHT: & str = "audio/SFX/SFX_Move_LeftRight.wav";

pub const MUSIC_MAIN_THEME: &str = "audio/ThemeWIP.wav";

pub const AUDIO_MUSIC_CHANNEL: & str = "MUSIC_CHAN";
pub const AUDIO_EFFECTS_CHANNEL: & str = "EFFECTS_CHAN";
pub const AUDIO_INTERFACE_CHANNEL: & str = "INTERFACE_CHAN";

pub const DEFAULT_MUSIC_VOLUME: f32 = 0.7;
pub const DEFAULT_EFFECT_VOLUME: f32 = 0.6;
pub const DEFAULT_INTERFACE_VOLUME: f32 = 0.9;
