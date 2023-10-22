use crate::player::movement::events::Grounded;
use crate::player::movement::systems::*;
use bevy::prelude::*;

pub mod components;
mod events;
pub mod systems;

pub struct MovementPlugin;

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum UpdateSet {
    HorizontalMovement,
    VerticalMovement,
    HorizontalConfinement,
    VerticalConfinement,
    ChangeDetection,
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        use crate::player::movement::UpdateSet::*;

        app.add_event::<Grounded>()
            .add_systems(Update, horizontal_movement.in_set(HorizontalMovement))
            .add_systems(
                Update,
                horizontal_collision_response.in_set(HorizontalConfinement),
            )
            .add_systems(
                Update,
                (jump, gaze, crouch, vertical_movement)
                    .chain()
                    .in_set(VerticalMovement),
            )
            .add_systems(
                Update,
                (vertical_collision_response, confine_in_window).in_set(VerticalConfinement),
            )
            .add_systems(Update, (coyote_jump, reset_coyote_jump))
            .configure_set(Update, HorizontalMovement.before(HorizontalConfinement))
            .configure_set(Update, VerticalMovement.before(VerticalConfinement))
            .configure_set(Update, HorizontalConfinement.before(VerticalMovement))
            .configure_set(Update, HorizontalConfinement.before(ChangeDetection))
            .configure_set(Update, VerticalConfinement.before(ChangeDetection));
    }
}
