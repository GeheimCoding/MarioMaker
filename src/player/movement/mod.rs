use crate::player::movement::systems::*;
use bevy::prelude::*;

pub mod components;
mod systems;

pub struct MovementPlugin;

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum UpdateSet {
    Movement,
    Confinement,
    ChangeDetection,
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                horizontal_movement,
                horizontal_collision_response,
                jump,
                vertical_movement,
                vertical_collision_response,
            )
                .chain()
                .in_set(UpdateSet::Movement),
        )
        .add_systems(Update, confine_in_window.in_set(UpdateSet::Confinement))
        .add_systems(Update, (coyote_jump, reset_coyote_jump))
        .configure_set(Update, UpdateSet::Movement.before(UpdateSet::Confinement))
        .configure_set(
            Update,
            UpdateSet::Confinement.before(UpdateSet::ChangeDetection),
        );
    }
}
