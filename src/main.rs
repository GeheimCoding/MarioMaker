use crate::character::CharacterPlugin;
use crate::events::Grounded;
use crate::resources::MousePosition;
use crate::system_sets::SystemSetPlugin;
use crate::systems::*;
use crate::world::WorldPlugin;
use bevy::prelude::*;
use bevy::window::close_on_esc;

mod character;
mod components;
mod content_manager;
mod events;
mod resources;
mod system_sets;
mod systems;
mod world;

fn main() {
    App::new()
        .init_resource::<MousePosition>()
        .insert_resource(ClearColor(Color::CYAN))
        .add_event::<Grounded>()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Mario Maker".to_owned(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    asset_folder: "content".to_owned(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            WorldPlugin,
            CharacterPlugin,
            SystemSetPlugin,
        ))
        .add_systems(Startup, setup_camera)
        .add_systems(
            Update,
            (animate, apply_gravity, update_mouse_position, close_on_esc),
        )
        .run();
}
