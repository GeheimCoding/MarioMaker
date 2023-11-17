use crate::resources::AppState;
use crate::world::resources::Tiles;
use crate::world::systems::*;
use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Tiles>()
            .add_systems(PreStartup, init)
            .add_systems(Startup, spawn)
            .add_systems(OnEnter(AppState::Editor), spawn_preview_block)
            .add_systems(OnExit(AppState::Editor), despawn_preview_block)
            .add_systems(
                Update,
                (move_preview_block, handle_block_placement).run_if(in_state(AppState::Editor)),
            );
    }
}
