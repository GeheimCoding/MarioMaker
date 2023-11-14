use crate::components::Collider;
use crate::content_manager::{TextureData, Textures};
use crate::resources::MousePosition;
use crate::world::components::{Block, PreviewBlock, TILE_SIZE};
use crate::world::resources::{Texture, Tiles};
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

pub fn spawn(mut commands: Commands, mut tiles: ResMut<Tiles>, textures: Res<Textures<Texture>>) {
    let texture_handle = textures.get(&Texture::Block);
    for x in -10..20 {
        spawn_block(&mut commands, &mut tiles, texture_handle.clone(), x, -4);
    }
    spawn_block(&mut commands, &mut tiles, texture_handle.clone(), 10, -3);
    spawn_block(&mut commands, &mut tiles, texture_handle.clone(), -8, -3);
}

pub fn spawn_preview_block(mut commands: Commands, textures: Res<Textures<Texture>>) {
    commands.spawn((
        PreviewBlock,
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
            texture_atlas: textures.get(&Texture::Block),
            ..default()
        },
    ));
}

pub fn move_preview_block(
    mouse_position: Res<MousePosition>,
    mut query: Query<&mut Transform, With<PreviewBlock>>,
) {
    // this prevents the block from instantly moving to the center without any mouse movement
    if mouse_position.0 == Vec2::ZERO {
        return;
    }
    let mut transform = query.single_mut();
    let coords = (mouse_position.0 + TILE_SIZE / 2.0) / TILE_SIZE;
    transform.translation = Vec3::new(
        coords.x.floor() * TILE_SIZE,
        coords.y.floor() * TILE_SIZE,
        0.0,
    );
}

pub fn handle_block_placement(
    mut commands: Commands,
    mut tiles: ResMut<Tiles>,
    textures: Res<Textures<Texture>>,
    mouse_position: Res<MousePosition>,
    mouse_input: Res<Input<MouseButton>>,
) {
    let mouse_coords = (mouse_position.0 + TILE_SIZE / 2.0) / TILE_SIZE;
    let coords = (
        mouse_coords.x.floor() as isize,
        mouse_coords.y.floor() as isize,
    );
    if mouse_input.pressed(MouseButton::Left) && !tiles.contains_key(&coords) {
        spawn_block(
            &mut commands,
            &mut tiles,
            textures.get(&Texture::Block),
            coords.0,
            coords.1,
        )
    } else if mouse_input.pressed(MouseButton::Right) && tiles.contains_key(&coords) {
        commands.entity(tiles.remove(&coords).unwrap()).despawn();
    }
}

fn spawn_block(
    commands: &mut Commands,
    tiles: &mut Tiles,
    texture_atlas: Handle<TextureAtlas>,
    x: isize,
    y: isize,
) {
    let entity = commands.spawn((
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
    tiles.insert((x, y), entity.id());
}

fn position_to_translation(position: Vec2) -> Vec3 {
    Vec3::new(position.x, position.y, 0.0)
}
