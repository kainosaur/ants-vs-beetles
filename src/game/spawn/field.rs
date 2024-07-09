use bevy::prelude::*;

use crate::{core::asset::GameAssets, screen::Screen};

// 
pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_field);
}
// Step 1: Spawn a field with the grass sprite

#[derive(Debug, Event)]
pub(crate) struct SpawnField;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Field;

fn spawn_field(
    _trigger: Trigger<SpawnField>,
    mut commands: Commands,
    game_assets: ResMut<GameAssets>,
) {
    let texture_handle = game_assets.grass.clone();
    let field_size = Vec2::new(64.0, 64.0);
    let grid_size = IVec2::new(9, 5);
    let start_position = Vec2::new(
        -grid_size.x as f32 / 2.0 * field_size.x,
        -grid_size.y as f32 / 2.0 * field_size.y
    );
    for y in 0..grid_size.y {
        for x in 0..grid_size.x {
            let position = Vec3::new(
                start_position.x + x as f32 * field_size.x,
                start_position.y + y as f32 * field_size.y,
                0.0,
            );

            commands.spawn((Field, SpriteBundle {
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
        }
    }
}

// Step 2: Allow ants to be placed on field without overlapping