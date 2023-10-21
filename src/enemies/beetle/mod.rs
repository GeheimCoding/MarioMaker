use crate::enemies::beetle::resources::Animations;
use crate::enemies::beetle::systems::{init, spawn};
use bevy::prelude::*;

mod components;
mod resources;
mod systems;

pub struct BeetlePlugin;

impl Plugin for BeetlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Animations>()
            .add_systems(PreStartup, init)
            .add_systems(Startup, spawn);
    }
}
