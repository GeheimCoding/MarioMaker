use crate::editor::EditorPlugin;
use crate::level::LevelPlugin;
use crate::resources::{AppState, MousePosition};
use crate::system_sets::SystemSetPlugin;
use crate::systems::*;
use crate::world::WorldPlugin;
use bevy::prelude::*;
use bevy::window::close_on_esc;

mod components;
mod content_manager;
mod editor;
mod level;
mod resources;
mod system_sets;
mod systems;
mod world;

fn main() {
    App::new()
        .add_state::<AppState>()
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
            LevelPlugin,
            SystemSetPlugin,
            EditorPlugin,
        ))
        .add_systems(Startup, setup_cameras)
        .add_systems(OnEnter(AppState::Editor), spawn_cursor)
        .add_systems(OnExit(AppState::Editor), despawn_cursor)
        .add_systems(
            Update,
            (
                animate,
                close_on_esc,
                change_state,
                update_mouse_position,
                move_cursor.run_if(in_state(AppState::Editor)),
                apply_gravity.run_if(in_state(AppState::Level)),
                update_cursor_sprite.run_if(in_state(AppState::Editor)),
            ),
        )
        .run();
}
