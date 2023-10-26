use crate::characters::player::movement::systems::*;
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
                (jump, gaze, crouch).chain().in_set(VerticalMovementActions),
            )
            .add_systems(Update, (coyote_jump, reset_coyote_jump));
    }
}
