//! Spawn the player.

use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::core::asset::GameAssets;
use crate::{
    // game::physics::{PhysicalTransform, Velocity},
    screen::Screen,
};

// Looks for the "spawn_cards" trigger to be called
pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_cards);
}

#[derive(Debug, Event)]
pub(crate) struct SpawnCards;

#[derive(Debug, PartialEq, Eq, Hash, Component, Clone, Copy)]
pub(crate) enum CardTypes {
    Ant,
    IceAnt,
    Warden,
    FireAnt,
}

pub const CARD_TYPES: &[CardTypes] = &[
    CardTypes::Ant,
    CardTypes::IceAnt,
    CardTypes::Warden,
    CardTypes::FireAnt,
];

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Card;

const X_EXTENT: f32 = 750.0;
const CARD_Y_POSITION: f32 = 200.0;

fn spawn_cards(
    _trigger: Trigger<SpawnCards>,
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_assets: ResMut<GameAssets>,
) {
    let ant_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, None);
    let texture_atlas_ant_layout = texture_atlas_layouts.add(ant_layout);
    // Card for the ant to be on
    let card_backing = Mesh2dHandle(meshes.add(Rectangle::new(1.0, 1.0)));
    let ant_amount = CARD_TYPES.len();
    // Loop through all cards, spawn on screen
    for (i, card_type) in CARD_TYPES.iter().enumerate() {
        commands.spawn((Card,
            MaterialMesh2dBundle {
                mesh: card_backing.clone(),
                material: materials.add(Color::hsl(0.0,0.0,100.0)),
                transform: Transform::from_xyz(
                    -X_EXTENT / 2. + i as f32 / (ant_amount - 1) as f32 * X_EXTENT,
                    CARD_Y_POSITION,
                    1.0,
                ).with_scale(Vec3::new(
                    75.0,
                    100.,
                    1.0,
                )),
                ..default()
            },
            StateScoped(Screen::Playing),
        ))
        .with_children(|parent| {
            parent.spawn((*card_type, 
                // Sprite of the Ant
                SpriteBundle {
                    texture: game_assets.ants.clone(),
                    transform: Transform::from_xyz(
                        0.0,
                        0.0,
                        6.0
                    ).with_scale(Vec3::splat(
                        0.05
                    )),
                    ..default()
                },
                // Index of the ant
                TextureAtlas {
                    layout: texture_atlas_ant_layout.clone(),
                    index: i,
                },
                ));
            });
    }
}

