use crate::ui::systems::{spawn_level_timer, update_level_timer};
use bevy::prelude::*;

mod components;
mod systems;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_level_timer)
            .add_systems(Update, update_level_timer);
    }
}
