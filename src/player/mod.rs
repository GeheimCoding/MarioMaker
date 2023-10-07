use crate::player::resources::Animations;
use crate::player::systems::*;
use crate::systems::*;
use bevy::prelude::*;

mod components;
mod resources;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Animations>()
            .add_systems(PreStartup, init)
            .add_systems(Startup, spawn)
            .add_systems(
                Update,
                (
                    jump,
                    change_state,
                    change_animation,
                    vertical_movement.after(jump),
                    horizontal_movement.pipe(error_handler),
                    confine_in_window.after(vertical_movement),
                ),
            );
    }
}
