use crate::player::PlayerPlugin;
use crate::systems::*;
use bevy::prelude::*;

mod components;
mod player;
mod systems;

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
        ))
        .add_systems(Startup, setup_camera)
        .add_systems(Update, animate)
        .run();
}
