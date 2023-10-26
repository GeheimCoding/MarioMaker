use crate::characters::movement::systems::*;
use bevy::prelude::*;
mod systems;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        use crate::system_sets::UpdateSet::*;

        app.add_systems(Update, horizontal_movement.in_set(HorizontalMovement))
            .add_systems(
                Update,
                horizontal_block_collision_response.in_set(HorizontalConfinement),
            )
            .add_systems(Update, vertical_movement.in_set(VerticalMovement))
            .add_systems(
                Update,
                (vertical_block_collision_response, confine_in_window).in_set(VerticalConfinement),
            );
    }
}
