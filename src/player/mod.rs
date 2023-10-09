use crate::player::movement::{MovementPlugin, UpdateSet};
use crate::player::resources::Animations;
use crate::player::systems::*;
use bevy::prelude::*;

mod components;
mod movement;
mod resources;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Animations>()
            .add_plugins(MovementPlugin)
            .add_systems(PreStartup, init)
            .add_systems(Startup, spawn)
            .add_systems(
                Update,
                (handle_velocity_change, handle_state_change).in_set(UpdateSet::ChangeDetection),
            );
    }
}
