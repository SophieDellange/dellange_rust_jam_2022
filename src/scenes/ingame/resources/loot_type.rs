use std::path::Path;

use bevy::prelude::*;
use rand::{thread_rng, Rng};

const TEXTURES_PATH: &str = "textures";

pub enum LootType {
    GoldCoin,
    Bomb,
    Torch,
    KeyBlue,
}

const ALL_PARTS_TEXTURES: [&str; 8] = [
    "creature_cow_0.png",
    "creature_cow_1.png",
    "creature_monster_0.png",
    "creature_monster_1.png",
    "creature_rabbit_0.png",
    "creature_rabbit_1.png",
    "creature_talons_0.png",
    "creature_hearth.png"
];

impl LootType {
    pub fn texture(&self, asset_server: &Res<AssetServer>) -> Handle<Image> {
        let basename = match self {
            LootType::GoldCoin => "loot_gold_coin.png",
            LootType::Bomb => "loot_bomb.png",
            LootType::Torch => "loot_torch.png",
            LootType::KeyBlue => "loot_key_blue.png",
        };

        let full_path = Path::new(TEXTURES_PATH).join(basename);

        asset_server.load(full_path)
    }

    pub fn random() -> Self {
        match thread_rng().gen_range(0..4) {
            0 => LootType::GoldCoin,
            1 => LootType::Bomb,
            2 => LootType::Torch,
            3 => LootType::KeyBlue,
            _ => unreachable!(),
        }
    }

    pub fn player_extra_tile_texture(&self, asset_server: &AssetServer) -> Handle<Image> {
        let texture_range = match self {
            LootType::GoldCoin => 
               0..=1,
            LootType::Bomb => 2..=3,
            LootType::Torch =>  4..=5,
            LootType::KeyBlue => 6..=7,
        };

        let real_index = thread_rng().gen_range(texture_range);
        let full_path = Path::new(TEXTURES_PATH).join(ALL_PARTS_TEXTURES[real_index]);

        asset_server.load(full_path)
    }
}
