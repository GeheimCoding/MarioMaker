use crate::components::{Camera, Collider};
use crate::content_manager::{TextureData, Textures};
use crate::world::components::{Block, PreviewBlock, TILE_SIZE};
use crate::world::resources::Texture;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

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
    spawn_preview_block(&mut commands, texture_handle.clone());
}

// https://bevy-cheatbook.github.io/cookbook/cursor2world.html
pub fn move_preview_block(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut preview_block_query: Query<&mut Transform, With<PreviewBlock>>,
    camera_query: Query<(&bevy::render::camera::Camera, &GlobalTransform), With<Camera>>,
) {
    let window = window_query.single();
    let mut transform = preview_block_query.single_mut();
    let (camera, camera_transform) = camera_query.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let coords = (world_position + TILE_SIZE / 2.0) / TILE_SIZE;
        transform.translation = Vec3::new(
            coords.x.floor() * TILE_SIZE,
            coords.y.floor() * TILE_SIZE,
            0.0,
        );
    }
}

fn spawn_block(commands: &mut Commands, texture_atlas: Handle<TextureAtlas>, x: isize, y: isize) {
    commands.spawn((
        Block,
        Collider::with_size(Vec2::splat(TILE_SIZE)),
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

fn spawn_preview_block(commands: &mut Commands, texture_atlas: Handle<TextureAtlas>) {
    commands.spawn((
        PreviewBlock,
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                ..default()
            },
            texture_atlas,
            ..default()
        },
    ));
}

fn position_to_translation(position: Vec2) -> Vec3 {
    Vec3::new(position.x, position.y, 0.0)
}
