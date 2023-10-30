use crate::characters::enemies::EnemyPlugin;
use crate::characters::events::{GrabbedEvent, GroundedEvent, JumpedOnEvent, KickedEvent};
use crate::characters::movement::MovementPlugin;
use crate::characters::player::PlayerPlugin;
use crate::characters::systems::{grab, kick};
use crate::system_sets::UpdateSet::HorizontalMovementActions;
use bevy::prelude::*;

pub mod components;
mod enemies;
mod movement;
pub mod player;

pub mod events;
mod systems;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GroundedEvent>()
            .add_event::<JumpedOnEvent>()
            .add_event::<KickedEvent>()
            .add_event::<GrabbedEvent>()
            .add_plugins((PlayerPlugin, EnemyPlugin, MovementPlugin))
            .add_systems(Update, (grab, kick).in_set(HorizontalMovementActions));
    }
}
