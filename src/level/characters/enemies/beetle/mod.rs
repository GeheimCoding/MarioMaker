use crate::level::characters::enemies::beetle::resources::Animations;
use crate::level::characters::enemies::beetle::systems::*;
use crate::resources::AppState;
use bevy::prelude::*;

mod components;
mod resources;
mod systems;

pub struct BeetlePlugin;

impl Plugin for BeetlePlugin {
    fn build(&self, app: &mut App) {
        use crate::system_sets::UpdateSet::*;
        app.init_resource::<Animations>()
            .add_systems(Startup, init)
            .add_systems(OnEnter(AppState::Level), spawn)
            .add_systems(OnExit(AppState::Level), despawn)
            .add_systems(Update, (reset_jumpable, handle_grounded_event))
            .add_systems(
                Update,
                (
                    handle_velocity_change,
                    handle_state_change,
                    die,
                    get_kicked,
                    get_grabbed,
                )
                    .in_set(ChangeDetection),
            );
    }
}
