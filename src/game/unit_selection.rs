use bevy::prelude::*;

use super::spawn::cards::Card;

pub(super) fn plugin(app: &mut App) {
    app
    // .init_resource::<SpatialIndex>()
    .observe(select_ant)
    .add_systems(Update, handle_click);
}

#[derive(Debug, Event)]
pub(crate) struct SelectAnt{
    pos: Vec2,
}

#[derive(Component)]
pub(crate) struct SelectedAnt;

fn select_ant (
    trigger: Trigger<SelectAnt>,
    mut card: Query<&GlobalTransform, With<Card>>,
    mut unit_selected: Query<&SelectedAnt>,
) {
    let event = trigger.event();
    // query through cards, see if it matches with click position
    
    for (global_transform) in &mut card {
        let translation = global_transform.compute_transform().translation;
        let scale = global_transform.compute_transform().scale;
        let card_pos = Vec2::new(translation.x, translation.y);
        let card_size = Vec2::new(scale.x, scale.y);

        info!("Card size: {}, Card position: {}", card_size, card_pos);

        let max_x = translation.x + (scale.x / 2.0);
        let min_x = translation.x - (scale.x / 2.0);
        let max_y = translation.y + (scale.y / 2.0);
        let min_y = translation.y - (scale.y / 2.0);
            // TODO: Make conditionals here.
            // Check if: Player is clicking on a card
                // if so: check if: Player already has ant in hand
                    //if so: remove ant from hand (deselect ant)
                    // else: Place lower opacity mock ant duplicate in player cursor
            // Check if: Player has ant in "hand"
                //if so: player is clicking on "field" sprite
                    //if so: Place duplicate ant on field (flipped)
                //if not: remove ant from "hand"
        if event.pos.x >= min_x && event.pos.x <= max_x && event.pos.y >= min_y && event.pos.y <= max_y {
            info!("CLICK POS {} IS WITHIN MAX_X {}, MIN_X {}, MAX_Y {}, MIN_Y {}", event.pos, max_x, min_x, max_y, min_y);
        } else {
            info!("CLICK POS {} IS NOT WITHIN MAX_X {}, MIN_X {}, MAX_Y {}, MIN_Y {}", event.pos, max_x, min_x, max_y, min_y);
        }
    }
}

// fn create_ghost_unit (

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
    }
}