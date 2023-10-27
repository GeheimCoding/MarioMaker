use crate::characters::enemies::EnemyPlugin;
use crate::characters::events::{Grounded, JumpedOn};
use crate::characters::movement::MovementPlugin;
use crate::characters::player::PlayerPlugin;
use bevy::prelude::*;

mod components;
mod enemies;
mod movement;
pub mod player;

pub mod events;
mod systems;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Grounded>()
            .add_event::<JumpedOn>()
            .add_plugins((PlayerPlugin, EnemyPlugin, MovementPlugin));
    }
}
