use crate::characters::enemies::EnemyPlugin;
use crate::characters::movement::MovementPlugin;
use crate::characters::player::PlayerPlugin;
use bevy::prelude::*;

mod components;
mod enemies;
pub mod movement;
pub mod player;

pub mod systems;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, EnemyPlugin, MovementPlugin));
    }
}
