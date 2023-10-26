use crate::characters::enemies::EnemyPlugin;
use crate::characters::player::PlayerPlugin;
use crate::characters::systems::vertical_movement;
use crate::system_sets::UpdateSet::VerticalMovement;
use bevy::prelude::*;

mod components;
mod enemies;
mod movement;
pub mod player;
mod systems;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, EnemyPlugin))
            .add_systems(Update, vertical_movement.in_set(VerticalMovement));
    }
}
