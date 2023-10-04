use crate::player::resources::{Animations, Textures};
use crate::player::systems::*;
use crate::systems::*;
use bevy::prelude::*;

mod components;
mod resources;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Textures>()
            .init_resource::<Animations>()
            .add_systems(Startup, spawn)
            .add_systems(
                Update,
                (
                    change_state,
                    change_animation,
                    horizontal_movement.pipe(error_handler),
                    vertical_movement.pipe(error_handler),
                ),
            );
    }
}
