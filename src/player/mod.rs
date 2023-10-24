use crate::player::movement::MovementPlugin;
use crate::player::resources::Animations;
use crate::player::systems::*;
use crate::system_sets::UpdateSet;
use bevy::prelude::*;

mod components;
pub mod movement;
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
                (handle_velocity_change, handle_state_change, move_camera)
                    .in_set(UpdateSet::ChangeDetection),
            );
    }
}
