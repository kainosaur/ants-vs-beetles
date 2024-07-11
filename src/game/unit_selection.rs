use bevy::prelude::*;

use crate::{
    core::asset::GameAssets,
    game::spawn::{cards::UNITLIST, field::FIELD_SIZE},
};

use super::spawn::{
    cards::{spawn_unit, Card, UnitProperties, CARD_SIZE},
    field::Field,
};

pub(super) fn plugin(app: &mut App) {
    app
        // .init_resource::<SpatialIndex>()
        .observe(select_ant)
        .observe(place_unit)
        .add_systems(Update, handle_click);
}

#[derive(Debug, Event)]
pub(crate) struct SelectAnt {
    pos: Vec2,
}

#[derive(Debug, Event)]
pub(crate) struct PlaceUnit {
    pos: Vec2,
}

#[derive(Component)]
pub(crate) struct SelectedUnit;

// Checks if the click is within the bounds of the defined Entity
fn calculate_if_in_bounds(mouse_pos: Vec2, transform: Transform, object_bounds: Vec2) -> bool {
    let translation = transform.translation;
    let object_pos = Vec2::new(translation.x, translation.y);

    info!(
        "Card size: {}, Card position: {}",
        object_bounds, object_pos
    );
    let max_x = object_pos.x + (object_bounds.x / 2.0);
    let min_x = object_pos.x - (object_bounds.x / 2.0);
    let max_y = object_pos.y + (object_bounds.y / 2.0);
    let min_y = object_pos.y - (object_bounds.y / 2.0);

    if mouse_pos.x >= min_x && mouse_pos.x <= max_x && mouse_pos.y >= min_y && mouse_pos.y <= max_y
    {
        info!(
            "CLICK POS {} IS WITHIN MAX_X {}, MIN_X {}, MAX_Y {}, MIN_Y {}",
            mouse_pos, max_x, min_x, max_y, min_y
        );
        return true;
    } else {
        info!(
            "CLICK POS {} IS NOT WITHIN MAX_X {}, MIN_X {}, MAX_Y {}, MIN_Y {}",
            mouse_pos, max_x, min_x, max_y, min_y
        );
        return false;
    }
}

fn select_ant(
    trigger: Trigger<SelectAnt>,
    mut card_query: Query<(&Transform, Entity), With<Card>>,
    mut commands: Commands,
) {
    let mouse_click = trigger.event();
    // query through cards, see if it matches with click position

    for (transform, card_entity) in &mut card_query {
        let is_in_bounds: bool = calculate_if_in_bounds(
            Vec2::new(mouse_click.pos.x, mouse_click.pos.y),
            *transform,
            CARD_SIZE,
        );
        if is_in_bounds {
            commands.entity(card_entity).insert(SelectedUnit);
            info!("Entity has {}", commands.entity(card_entity).id());
        }
    }
}

fn place_unit(
    trigger: Trigger<PlaceUnit>,
    mut selected_query: Query<(&UnitProperties, Entity), With<SelectedUnit>>,
    mut selected_field: Query<(&Transform, Entity), With<Field>>,
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    game_assets: ResMut<GameAssets>,
) {
    let mouse_release = trigger.event();
    // Check if released click is on ANY field
    for (unit_properties, entity) in &mut selected_query {
        commands.entity(entity).remove::<SelectedUnit>();
        info!("Entity is {}", unit_properties.name.to_string());

        for (transform, field_entity) in &mut selected_field {
            // Checking if the mouse release is within the bounds
            let is_in_bounds: bool = calculate_if_in_bounds(
                Vec2::new(mouse_release.pos.x, mouse_release.pos.y),
                *transform,
                FIELD_SIZE,
            );
            if is_in_bounds {
                for (index, unit) in UNITLIST.iter().enumerate() {
                    // to find the index of ant that player was holding
                    if unit_properties.name == unit.name {
                        let ant_layout =
                            TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, None);
                        let texture_atlas_ant_layout = texture_atlas_layouts.add(ant_layout);
                        let unit_to_spawn = spawn_unit(
                            &mut commands,
                            &texture_atlas_ant_layout,
                            index,
                            &game_assets,
                        );
                        commands
                            .entity(field_entity)
                            .push_children(&[unit_to_spawn]);
                    }
                }
            }
        }
    }
}

// fn create_ghost_unit (
//     unit_selected: CardTypes,
//     mut commands: Commands,
// ) {

// }

// fn destroy_ghost_unit (

// ) {

// }

fn handle_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = camera.single();
    if let Some(pos) = windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            commands.trigger(SelectAnt { pos });
            info!("Mouse clicked!");
        }
        if mouse_button_input.just_released(MouseButton::Left) {
            commands.trigger(PlaceUnit { pos });
            info!("Mouse released!");
        }
    }
}
