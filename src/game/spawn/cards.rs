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

#[derive(Debug, PartialEq, Eq, Hash, Component, Clone)]
pub struct UnitProperties {
    pub name: &'static str,
}

pub const UNITLIST: &[UnitProperties] = &[
    UnitProperties { name: "Ant" },
    UnitProperties { name: "IceAnt" },
    UnitProperties { name: "Warden" },
    UnitProperties { name: "FireAnt" },
];

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Card;

const X_EXTENT: f32 = 750.0;
const CARD_Y_POSITION: f32 = 200.0;
pub(crate) const CARD_SIZE: Vec2 = Vec2::new(75.0, 100.0);

pub(crate) fn spawn_unit(
    commands: &mut Commands,
    texture_atlas_layout: &Handle<TextureAtlasLayout>,
    unit_index: usize,
    game_assets: &ResMut<GameAssets>,
) -> Entity {
    let unit_entity = commands
        .spawn((
            // Sprite of the Ant
            SpriteBundle {
                texture: game_assets.ants.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 6.0).with_scale(Vec3::splat(2.0)),
                ..default()
            },
            // Index of the ant
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: unit_index,
            },
        ))
        .id();
    return unit_entity;
}

fn spawn_cards(
    _trigger: Trigger<SpawnCards>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    game_assets: ResMut<GameAssets>,
) {
    let ant_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, None);
    let texture_atlas_ant_layout = texture_atlas_layouts.add(ant_layout);
    // Card for the ant to be on
    let card_backing = Mesh2dHandle(meshes.add(Rectangle::new(CARD_SIZE.x, CARD_SIZE.y)));
    let ant_amount = UNITLIST.len();
    // Loop through all cards, spawn on screen
    for (i, card_type) in UNITLIST.iter().enumerate() {
        let card = commands
            .spawn((
                Card,
                card_type.clone(),
                MaterialMesh2dBundle {
                    mesh: card_backing.clone(),
                    material: materials.add(Color::hsl(0.0, 0.0, 100.0)),
                    transform: Transform::from_xyz(
                        -X_EXTENT / 2. + i as f32 / (ant_amount - 1) as f32 * X_EXTENT,
                        CARD_Y_POSITION,
                        1.0,
                    ),
                    ..default()
                },
                StateScoped(Screen::Playing),
            ))
            .id();
        // get unit entity and push unit as child of card
        let unit = spawn_unit(&mut commands, &texture_atlas_ant_layout, i, &game_assets);
        commands.entity(card).push_children(&[unit]);
    }
}
