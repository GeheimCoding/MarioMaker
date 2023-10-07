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
    commands.spawn((
        Block,
        SpriteSheetBundle {
            texture_atlas: textures.get(&Texture::Block),
            transform: Transform::from_translation(position_to_translation(get_tile_position(
                1, 0,
            ))),
            ..default()
        },
    ));
}

fn get_tile_position(x: isize, y: isize) -> Vec2 {
    Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE)
}

fn position_to_translation(position: Vec2) -> Vec3 {
    Vec3::new(position.x, position.y, 0.0)
}
