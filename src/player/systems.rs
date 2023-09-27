use crate::components::{AnimationTimer, Direction};
use crate::player::components::Player;
use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/mario.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 24.0), 2, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        Player { speed: 80.0 },
        Direction::Right,
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
    ));
}

pub fn horizontal_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Direction, &Player)>,
) -> Result<(), QuerySingleError> {
    let (mut transform, mut player_direction, player) = query.get_single_mut()?;

    let direction = get_direction(keyboard_input);
    if direction.x < 0.0 {
        *player_direction = Direction::Left;
    } else if direction.x > 0.0 {
        *player_direction = Direction::Right;
    }
    transform.translation += direction * player.speed * time.delta_seconds();

    Ok(())
}

pub fn animate(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Direction), With<Player>>,
) -> Result<(), QuerySingleError> {
    let (mut timer, mut sprite, player_direction) = query.get_single_mut()?;

    timer.tick(time.delta());
    if timer.just_finished() {
        sprite.index = (sprite.index + 1) % 2;
    }
    sprite.flip_x = player_direction == &Direction::Left;

    Ok(())
}

fn get_direction(keyboard_input: Res<Input<KeyCode>>) -> Vec3 {
    let mut direction = Vec2::ZERO;
    if keyboard_input.any_pressed(vec![KeyCode::Left, KeyCode::A]) {
        direction.x -= 1.0;
    }
    if keyboard_input.any_pressed(vec![KeyCode::Right, KeyCode::D]) {
        direction.x += 1.0;
    }
    Vec3::new(direction.x, direction.y, 0.0)
}
