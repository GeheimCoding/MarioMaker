use crate::characters::enemies::EnemyPlugin;
use crate::characters::player::PlayerPlugin;
use crate::characters::systems::*;
use bevy::prelude::*;

mod components;
mod enemies;
mod movement;
pub mod player;
mod systems;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        use crate::system_sets::UpdateSet::*;

        app.add_plugins((PlayerPlugin, EnemyPlugin))
            .add_systems(Update, horizontal_movement.in_set(HorizontalMovement))
            .add_systems(Update, vertical_movement.in_set(VerticalMovement));
    }
}
