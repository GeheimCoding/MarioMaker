use crate::enemies::beetle::resources::Animations;
use crate::enemies::beetle::systems::{
    init, spawn, vertical_collision_response, vertical_movement,
};
use bevy::prelude::*;

mod components;
mod resources;
mod systems;

pub struct BeetlePlugin;

impl Plugin for BeetlePlugin {
    fn build(&self, app: &mut App) {
        use crate::player::movement::UpdateSet::*;

        app.init_resource::<Animations>()
            .add_systems(PreStartup, init)
            .add_systems(Startup, spawn)
            .add_systems(Update, vertical_movement.in_set(VerticalMovement))
            .add_systems(
                Update,
                vertical_collision_response.in_set(VerticalConfinement),
            );
    }
}
