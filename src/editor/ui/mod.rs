use crate::editor::ui::systems::*;
use crate::resources::AppState;
use bevy::prelude::*;

mod components;
mod systems;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Level), spawn_level_timer)
            .add_systems(OnExit(AppState::Level), despawn_level_timer)
            .add_systems(Update, update_level_timer.run_if(in_state(AppState::Level)));
    }
}
