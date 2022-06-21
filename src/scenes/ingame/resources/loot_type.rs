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

}
