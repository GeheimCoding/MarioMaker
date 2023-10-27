use crate::characters::enemies::beetle::resources::Animations;
use crate::characters::enemies::beetle::systems::*;
use bevy::prelude::*;

mod components;
mod resources;
mod systems;

pub struct BeetlePlugin;

impl Plugin for BeetlePlugin {
    fn build(&self, app: &mut App) {
        use crate::system_sets::UpdateSet::*;
        app.init_resource::<Animations>()
            .add_systems(PreStartup, init)
            .add_systems(Startup, spawn)
            .add_systems(Update, handle_velocity_change.in_set(ChangeDetection))
            .add_systems(Update, die);
    }
}
