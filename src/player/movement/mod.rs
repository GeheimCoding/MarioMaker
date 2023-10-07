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
            (jump, horizontal_movement, vertical_movement.after(jump)).in_set(UpdateSet::Movement),
        )
        .add_systems(Update, confine_in_window.in_set(UpdateSet::Confinement))
        .configure_set(Update, UpdateSet::Movement.before(UpdateSet::Confinement))
        .configure_set(
            Update,
            UpdateSet::Confinement.before(UpdateSet::ChangeDetection),
        );
    }
}
