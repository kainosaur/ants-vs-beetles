//! Asset loading and management.

use bevy::{asset::AssetMetaCheck, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app
    .add_plugins(AssetPlugin {
        // Wasm builds will check for meta files (that don't exist) if this isn't set.
        // This causes errors and even panics on web build on itch.
        // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
        meta_check: AssetMetaCheck::Never,
        ..default()
    })
    .init_resource::<GameAssets>()
    .add_systems(Startup, load_assets);
}

#[derive(Resource, Default)]
pub struct GameAssets {
    pub grass: Handle<Image>,
    pub ants: Handle<Image>,
}

fn load_assets(
    mut game_assets: ResMut<GameAssets>,
    asset_server: Res<AssetServer>
) {
    *game_assets = GameAssets {
        grass: asset_server.load("grass.png"),
        ants: asset_server.load("ants.png"),
    };
}