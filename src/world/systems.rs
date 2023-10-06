use crate::world::components::{Block, TILE_SIZE};
use crate::world::resources::{Texture, Textures};
use bevy::prelude::*;

pub fn spawn(mut commands: Commands, textures: Res<Textures>) {
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
