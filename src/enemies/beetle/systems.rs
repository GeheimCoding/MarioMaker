use crate::components::Direction;
use crate::content_manager::{TextureData, Textures};
use crate::enemies::beetle::components::{Beetle, State};
use crate::enemies::beetle::resources::{Animations, Texture};
use bevy::prelude::*;

pub fn init(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    Textures::init(
        commands,
        asset_server,
        texture_atlases,
        vec![TextureData {
            texture: Texture::Beetle,
            path: "textures/beetle.png".to_owned(),
            tile_size: Vec2::new(16.0, 16.0),
            columns: 6,
            rows: 1,
        }],
    );
}

pub fn spawn(
    mut commands: Commands,
    textures: Res<Textures<Texture>>,
    animations: Res<Animations>,
) {
    commands.spawn((
        Beetle,
        State::IdleAlive,
        Direction::Left,
        SpriteSheetBundle {
            texture_atlas: textures.get(&Texture::Beetle),
            ..default()
        },
        animations.get(&State::IdleAlive),
    ));
}
