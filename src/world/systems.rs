use crate::content_manager::{TextureData, Textures};
use crate::world::components::{Block, TILE_SIZE};
use crate::world::resources::Texture;
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
            texture: Texture::Block,
            path: "textures/block.png".to_owned(),
            tile_size: Vec2::splat(TILE_SIZE),
            columns: 1,
            rows: 1,
        }],
    );
}

pub fn spawn(mut commands: Commands, textures: Res<Textures<Texture>>) {
    let texture_handle = textures.get(&Texture::Block);
    for x in -2..4 {
        spawn_block(&mut commands, texture_handle.clone(), x, -3);
    }
    spawn_block(&mut commands, texture_handle.clone(), 1, -2);
    spawn_block(&mut commands, texture_handle.clone(), 2, -2);
    spawn_block(&mut commands, texture_handle.clone(), 2, -1);
}

fn spawn_block(commands: &mut Commands, texture_atlas: Handle<TextureAtlas>, x: isize, y: isize) {
    commands.spawn((
        Block,
        SpriteSheetBundle {
            texture_atlas,
            transform: Transform::from_translation(position_to_translation(Vec2::new(
                x as f32 * TILE_SIZE,
                y as f32 * TILE_SIZE,
            ))),
            ..default()
        },
    ));
}

fn position_to_translation(position: Vec2) -> Vec3 {
    Vec3::new(position.x, position.y, 0.0)
}
