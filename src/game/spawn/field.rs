use bevy::prelude::*;

use crate::{core::asset::GameAssets, screen::Screen};

//
pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_field);
}

// #[derive(Resource)]
// pub struct FieldEntities {
//     pub entities: Vec<Entity>,
// }

// impl FieldEntities {
//     fn register_field() -> Self {
//         FieldEntities {
//             entities: Vec::new(),
//         }
//     }
// }

#[derive(Debug, Event)]
pub(crate) struct SpawnField;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Field;

pub(crate) const FIELD_SIZE: Vec2 = Vec2::splat(64.0);

fn spawn_field(
    _trigger: Trigger<SpawnField>,
    mut commands: Commands,
    game_assets: ResMut<GameAssets>,
    // mut field_entities: ResMut<FieldEntities>,
) {
    let texture_handle = game_assets.grass.clone();
    let field_size = FIELD_SIZE;
    let grid_size = IVec2::new(9, 5);
    let start_position = Vec2::new(
        -grid_size.x as f32 / 2.0 * field_size.x,
        -grid_size.y as f32 / 2.0 * field_size.y,
    );
    for y in 0..grid_size.y {
        for x in 0..grid_size.x {
            let position = Vec3::new(
                start_position.x + x as f32 * field_size.x,
                start_position.y + y as f32 * field_size.y,
                0.0,
            );

            commands.spawn((
                Field,
                SpriteBundle {
                    texture: texture_handle.clone(),
                    transform: Transform::from_translation(position),
                    sprite: Sprite {
                        custom_size: Some(field_size),
                        ..default()
                    },
                    ..default()
                },
                StateScoped(Screen::Playing),
            ));
            // field_entities.entities.push(entity);
        }
    }
    // print_field_entites(field_entities);
}

// fn print_field_entites(
//     field_entities: Res<FieldEntities>,
// ) {
//     for entity in &field_entities.entities {
//         println!("Field entity: {:?}", entity);
//     }
// }

// Step 2: Allow ants to be placed on field without overlapping
