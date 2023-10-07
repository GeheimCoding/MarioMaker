use crate::player::PlayerPlugin;
use crate::systems::*;
use crate::world::WorldPlugin;
use bevy::prelude::*;

mod components;
mod content_manager;
mod player;
mod systems;
mod world;

fn main() {
    App::new()
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
        .add_systems(Update, (animate, apply_gravity))
        .run();
}
