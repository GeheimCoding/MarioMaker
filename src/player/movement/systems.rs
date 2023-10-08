use crate::components::{Collider, Direction, Velocity};
use crate::player::components::Player;
use crate::player::movement::components::{Acceleration, Jumping};
use bevy::prelude::*;

pub fn horizontal_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Direction, &mut Velocity, &Acceleration), With<Player>>,
) {
    let (mut transform, mut player_direction, mut player_velocity, acceleration) =
        query.single_mut();
    let direction = get_horizontal_direction(keyboard_input);
    let is_moving = direction != 0.0;
    let max_velocity = player_velocity.max.x;
    let velocity = &mut player_velocity.value.x;
    let acceleration = acceleration.0 * time.delta_seconds();

    if *velocity == 0.0 && !is_moving {
        return;
    }
    if direction < 0.0 {
        *player_direction = Direction::Left;
    } else if direction > 0.0 {
        *player_direction = Direction::Right;
    }

    *velocity = if is_moving {
        let factor = if direction.signum() == velocity.signum() {
            1.0
        } else {
            3.0
        };
        (*velocity + direction * acceleration * factor).clamp(-max_velocity, max_velocity)
    } else {
        apply_friction(*velocity, acceleration * 1.2)
    };
    transform.translation.x += *velocity * time.delta_seconds();
}

pub fn vertical_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Player>>,
) {
    let (mut transform, velocity) = query.single_mut();
    transform.translation.y += velocity.value.y * time.delta_seconds();
}

pub fn jump(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut Velocity), (With<Player>, Without<Jumping>)>,
) {
    if query.is_empty() {
        return;
    }
    if keyboard_input.any_just_pressed(vec![KeyCode::Space, KeyCode::Up, KeyCode::W]) {
        let (entity, mut velocity) = query.single_mut();
        commands.entity(entity).insert(Jumping);
        velocity.value.y = 250.0;
    }
}

pub fn confine_in_window(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Collider, &mut Transform, &mut Velocity), With<Player>>,
    camera_query: Query<&OrthographicProjection, With<Camera>>,
) {
    let (player, collider, mut player_transform, mut velocity) = player_query.single_mut();
    let camera_rect = camera_query.single().area;
    let player_rect = collider.get_rect(&player_transform);
    let position_response = collider.position_response(&camera_rect);

    if player_rect.min.x < camera_rect.min.x {
        velocity.value.x = 0.0;
        player_transform.translation.x = position_response.min.x;
    } else if player_rect.max.x > camera_rect.max.x {
        velocity.value.x = 0.0;
        player_transform.translation.x = position_response.max.x;
    }
    if player_rect.min.y < camera_rect.min.y {
        velocity.value.y = 0.0;
        player_transform.translation.y = position_response.min.y;
        commands.entity(player).remove::<Jumping>();
    } else if player_rect.max.y > camera_rect.max.y {
        velocity.value.y = 0.0;
        player_transform.translation.y = position_response.max.y;
    }
}

fn get_horizontal_direction(keyboard_input: Res<Input<KeyCode>>) -> f32 {
    let mut direction = 0.0;
    if keyboard_input.any_pressed(vec![KeyCode::Left, KeyCode::A]) {
        direction -= 1.0;
    }
    if keyboard_input.any_pressed(vec![KeyCode::Right, KeyCode::D]) {
        direction += 1.0;
    }
    direction
}

fn apply_friction(velocity: f32, friction: f32) -> f32 {
    if velocity > 0.0 {
        (velocity - friction).max(0.0)
    } else if velocity < 0.0 {
        (velocity + friction).min(0.0)
    } else {
        0.0
    }
}
