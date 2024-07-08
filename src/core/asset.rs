//! Asset loading and management.

use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(AssetPlugin {
        // Wasm builds will check for meta files (that don't exist) if this isn't set.
        // This causes errors and even panics on web build on itch.
        // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
        meta_check: AssetMetaCheck::Never,
        ..default()
    });
}

#[derive(Resource, Debug, Default)]
pub struct GameAssets {
    pub grass: Handle<Image>,
    pub ants: Handle<Image>,
}

fn load_assets(
    mut game_assets: ResMut<GameAssets>,
    asset_server: Res<AssetServer>,
) {
    *asset_server = GameAssets {
        grass: asset_server.load("grass.png"),
        ants: asset_server.load("ants.png")
    };
}