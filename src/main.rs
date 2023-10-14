use crate::player::PlayerPlugin;
use crate::resources::MousePosition;
use crate::systems::*;
use crate::world::WorldPlugin;
use bevy::prelude::*;
use bevy::window::close_on_esc;

mod components;
mod content_manager;
pub mod player;
mod resources;
mod systems;
mod world;

fn main() {
    App::new()
        .init_resource::<MousePosition>()
        .insert_resource(ClearColor(Color::CYAN))
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
            PlayerPlugin,
            WorldPlugin,
        ))
        .add_systems(Startup, setup_camera)
        .add_systems(
            Update,
            (animate, apply_gravity, update_mouse_position, close_on_esc),
        )
        .run();
}
