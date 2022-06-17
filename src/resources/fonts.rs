use bevy::{asset::LoadState, prelude::*};

pub struct Fonts {
    pub dogica: Handle<Font>,
}

impl Fonts {
    #[must_use]
    pub fn load(asset_server: &Res<AssetServer>) -> Fonts {
        Fonts {
            dogica: asset_server.load("fonts/dogica/dogicapixel.ttf"),
        }
    }

    #[must_use]
    pub fn all_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        asset_server.get_load_state(self.dogica.clone()) == LoadState::Loaded
    }
}
