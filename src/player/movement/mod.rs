use crate::player::movement::systems::*;
use bevy::prelude::*;

pub mod components;
mod systems;

pub struct MovementPlugin;

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum UpdateSet {
    Movement,
    ChangeDetection,
    CollisionDetection,
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (jump, horizontal_movement, vertical_movement.after(jump)).in_set(UpdateSet::Movement),
        )
        .add_systems(
            Update,
            (confine_in_window, collision_detection).in_set(UpdateSet::CollisionDetection),
        )
        .configure_set(
            Update,
            UpdateSet::Movement.before(UpdateSet::CollisionDetection),
        )
        .configure_set(
            Update,
            UpdateSet::CollisionDetection.before(UpdateSet::ChangeDetection),
        );
    }
}
