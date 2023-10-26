use crate::character::player::movement::systems::*;
use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        use crate::system_sets::UpdateSet::*;
        app.add_systems(Update, run.in_set(HorizontalMovementActions))
            .add_systems(
                Update,
                horizontal_collision_response.in_set(HorizontalConfinement),
            )
            .add_systems(
                Update,
                (jump, gaze, crouch).chain().in_set(VerticalMovementActions),
            )
            .add_systems(Update, vertical_movement.in_set(VerticalMovement))
            .add_systems(
                Update,
                (vertical_collision_response, confine_in_window).in_set(VerticalConfinement),
            )
            .add_systems(Update, (coyote_jump, reset_coyote_jump));
    }
}
