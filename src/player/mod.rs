use crate::player::resources::Animations;
use crate::player::systems::*;
use crate::systems::*;
use bevy::prelude::*;

mod components;
mod resources;
mod systems;

pub struct PlayerPlugin;

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
enum UpdateSet {
    Movement,
    Confinement,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Animations>()
            .add_systems(PreStartup, init)
            .add_systems(Startup, spawn)
            .add_systems(
                Update,
                (
                    jump,
                    vertical_movement.after(jump),
                    horizontal_movement.pipe(error_handler),
                )
                    .in_set(UpdateSet::Movement),
            )
            .add_systems(Update, confine_in_window.in_set(UpdateSet::Confinement))
            .add_systems(Update, (change_state, change_animation))
            .configure_set(Update, UpdateSet::Movement.before(UpdateSet::Confinement));
    }
}
