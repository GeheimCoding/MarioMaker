use crate::level::characters::enemies::EnemyPlugin;
use crate::level::characters::events::{GrabbedEvent, GroundedEvent, JumpedOnEvent, KickedEvent};
use crate::level::characters::movement::MovementPlugin;
use crate::level::characters::player::PlayerPlugin;
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
            .add_plugins((PlayerPlugin, EnemyPlugin, MovementPlugin));
    }
}
