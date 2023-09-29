use crate::player::systems::*;
use crate::systems::*;
use bevy::prelude::*;

mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, horizontal_movement.pipe(error_handler));
    }
}
