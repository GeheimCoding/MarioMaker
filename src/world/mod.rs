use crate::world::resources::Textures;
use crate::world::systems::spawn;
use bevy::prelude::*;

mod components;
mod resources;
mod systems;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Textures>().add_systems(Startup, spawn);
    }
}
