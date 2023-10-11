use crate::world::systems::*;
use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, init)
            .add_systems(Startup, spawn)
            .add_systems(Update, move_preview_block);
    }
}
