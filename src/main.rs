use crate::characters::CharacterPlugin;
use crate::resources::MousePosition;
use crate::system_sets::SystemSetPlugin;
use crate::systems::*;
use crate::ui::UiPlugin;
use crate::world::WorldPlugin;
use bevy::prelude::*;
use bevy::window::close_on_esc;

mod characters;
mod components;
mod content_manager;
mod resources;
mod system_sets;
mod systems;
mod ui;
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
                    file_path: "content".to_owned(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            WorldPlugin,
            CharacterPlugin,
            SystemSetPlugin,
            UiPlugin,
        ))
        .add_systems(Startup, setup_cameras)
        .add_systems(
            Update,
            (animate, apply_gravity, update_mouse_position, close_on_esc),
        )
        .run();
}
