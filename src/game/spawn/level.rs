//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use super::{field::SpawnField, cards::SpawnCards};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Debug, Event)]
pub(crate) struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    commands.trigger(SpawnCards);
    commands.trigger(SpawnField);
}
