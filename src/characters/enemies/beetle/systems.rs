use crate::characters::enemies::beetle::components::{Beetle, State};
use crate::characters::enemies::beetle::resources::{Animations, Texture};
use crate::characters::player::movement::systems::{
    is_colliding, respond_to_horizontal_collision, respond_to_vertical_collision,
};
use crate::components::{Collider, Direction, Gravity, Velocity};
use crate::content_manager::{TextureData, Textures};
use crate::world::components::Block;
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
        State::Walking,
        Direction::Right,
        Collider {
            size: Vec2::splat(16.0),
            offset: Vec2::ZERO,
        },
        Velocity {
            value: Vec2::new(50.0, 0.0),
            max: Vec2::new(50.0, 400.0),
        },
        Gravity(1200.0),
        SpriteSheetBundle {
            texture_atlas: textures.get(&Texture::Beetle),
            transform: Transform::from_translation(Vec3::new(96.0, 0.0, 0.0)),
            ..default()
        },
        animations.get(&State::Walking),
    ));
}

pub fn horizontal_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Beetle>>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.value.x * time.delta_seconds();
    }
}

pub fn vertical_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Beetle>>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.y += velocity.value.y * time.delta_seconds();
    }
}

pub fn horizontal_collision_response(
    mut beetle_query: Query<(&Collider, &mut Transform, &mut Velocity), With<Beetle>>,
    block_query: Query<(&Collider, &Transform), (With<Block>, Without<Beetle>)>,
) {
    let (beetle_collider, mut beetle_transform, mut velocity) = beetle_query.single_mut();

    for (block_collider, block_transform) in block_query.iter() {
        let beetle_rect = beetle_collider.get_rect(&beetle_transform);
        let block_rect = block_collider.get_rect(block_transform);

        if is_colliding(&beetle_rect, &block_rect) {
            let position_response = beetle_collider.position_response(&block_rect);
            respond_to_horizontal_collision(
                &mut beetle_transform,
                &mut Velocity::with_max(Vec2::ZERO),
                &beetle_rect,
                &block_rect,
                &position_response,
            );
            velocity.value.x *= -1.0;
        }
    }
}

pub fn vertical_collision_response(
    mut beetle_query: Query<(&Collider, &mut Transform, &mut Velocity), With<Beetle>>,
    block_query: Query<(&Collider, &Transform), (With<Block>, Without<Beetle>)>,
) {
    let (beetle_collider, mut beetle_transform, mut velocity) = beetle_query.single_mut();

    for (block_collider, block_transform) in block_query.iter() {
        let beetle_rect = beetle_collider.get_rect(&beetle_transform);
        let block_rect = block_collider.get_rect(block_transform);

        if is_colliding(&beetle_rect, &block_rect) {
            let position_response = beetle_collider.position_response(&block_rect);
            respond_to_vertical_collision(
                &mut beetle_transform,
                &mut velocity,
                &beetle_rect,
                &block_rect,
                &position_response,
            );
        }
    }
}

pub fn handle_velocity_change(
    mut query: Query<(&mut Direction, &Velocity), (With<Beetle>, Changed<Velocity>)>,
) {
    for (mut direction, velocity) in query.iter_mut() {
        if velocity.value.x < 0.0 {
            *direction = Direction::Left;
        } else if velocity.value.x > 0.0 {
            *direction = Direction::Right;
        }
    }
}
