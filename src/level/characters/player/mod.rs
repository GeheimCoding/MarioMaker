use crate::level::characters::player::movement::MovementPlugin;
use crate::level::characters::player::resources::Animations;
use crate::level::characters::player::systems::*;
use crate::resources::AppState;
use bevy::prelude::*;

pub mod components;
pub mod movement;
mod resources;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        use crate::system_sets::UpdateSet::*;

        app.init_resource::<Animations>()
            .add_plugins(MovementPlugin)
            .add_systems(Startup, init)
            .add_systems(OnEnter(AppState::Level), spawn)
            .add_systems(
                Update,
                (grab, kick, kick_held_item).in_set(HorizontalMovementActions),
            )
            .add_systems(
                Update,
                (
                    handle_velocity_change.before(handle_grabbed_sprite_variants),
                    handle_grabbed_sprite_variants.before(handle_state_change),
                    handle_state_change,
                    move_camera,
                    hold_item,
                    handle_kicked_event,
                    remove_kick_timer,
                )
                    .in_set(ChangeDetection),
            );
    }
}
