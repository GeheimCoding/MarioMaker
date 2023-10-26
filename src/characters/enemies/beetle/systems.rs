use crate::characters::components::{Character, CollisionResponse};
use crate::characters::enemies::beetle::components::{Beetle, State};
use crate::characters::enemies::beetle::resources::{Animations, Texture};
use crate::components::{Collider, Direction, Gravity, Velocity};
use crate::content_manager::{TextureData, Textures};
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
        Character,
        CollisionResponse {
            velocity: Vec2::new(50.0, 0.0),
        },
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
