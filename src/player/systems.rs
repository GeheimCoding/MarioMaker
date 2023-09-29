use crate::components::{Animation, Direction};
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
        Animation {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            frames: vec![0, 1],
            current_frame: 0,
        },
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
