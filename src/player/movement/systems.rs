use crate::components::{Collider, Direction, Velocity};
use crate::player::components::Player;
use crate::player::movement::components::{Acceleration, Airborne, CoyoteJump, JumpBuffer};
use crate::world::components::Block;
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
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<
        (
            Entity,
            &mut Velocity,
            Option<&Airborne>,
            Option<&mut JumpBuffer>,
        ),
        With<Player>,
    >,
) {
    let (entity, mut velocity, airborne, jump_buffer) = query.single_mut();
    let mut jump = |commands: &mut Commands| {
        commands.entity(entity).insert(Airborne);
        velocity.value.y = 250.0;
    };

    if keyboard_input.any_just_pressed(vec![KeyCode::Space, KeyCode::Up, KeyCode::W]) {
        if airborne.is_some() {
            commands
                .entity(entity)
                .insert(JumpBuffer(Timer::from_seconds(0.05, TimerMode::Once)));
        } else {
            jump(&mut commands);
        }
    }
    if let Some(mut jump_buffer) = jump_buffer {
        if !jump_buffer.0.tick(time.delta()).finished() && airborne.is_none() {
            commands.entity(entity).remove::<JumpBuffer>();
            jump(&mut commands);
        }
        if jump_buffer.0.finished() {
            commands.entity(entity).remove::<JumpBuffer>();
        }
    }
}

pub fn horizontal_collision_response(
    mut player_query: Query<(&Collider, &mut Transform, &mut Velocity), With<Player>>,
    block_query: Query<(&Collider, &Transform), (With<Block>, Without<Player>)>,
) {
    let (player_collider, mut player_transform, mut velocity) = player_query.single_mut();

    for (block_collider, block_transform) in block_query.iter() {
        let player_rect = player_collider.get_rect(&player_transform);
        let block_rect = block_collider.get_rect(block_transform);

        if is_colliding(&player_rect, &block_rect) {
            let position_response = player_collider.position_response(&block_rect);
            respond_to_horizontal_collision(
                &mut player_transform,
                &mut velocity,
                &player_rect,
                &block_rect,
                &position_response,
            );
        }
    }
}

pub fn vertical_collision_response(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Collider, &mut Transform, &mut Velocity), With<Player>>,
    block_query: Query<(&Collider, &Transform), (With<Block>, Without<Player>)>,
) {
    let (player, player_collider, mut player_transform, mut velocity) = player_query.single_mut();

    for (block_collider, block_transform) in block_query.iter() {
        let player_rect = player_collider.get_rect(&player_transform);
        let block_rect = block_collider.get_rect(block_transform);

        if is_colliding(&player_rect, &block_rect) {
            let position_response = player_collider.position_response(&block_rect);
            respond_to_vertical_collision(
                &mut player_transform,
                &mut velocity,
                &player_rect,
                &block_rect,
                &position_response,
            );
            if player_rect.max.y > block_rect.max.y {
                commands.entity(player).remove::<Airborne>();
            }
        }
    }
}

pub fn confine_in_window(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Collider, &mut Transform, &mut Velocity), With<Player>>,
    camera_query: Query<&OrthographicProjection, With<Camera>>,
) {
    let (player, collider, mut transform, mut velocity) = player_query.single_mut();
    let camera_rect = camera_query.single().area;
    let player_rect = collider.get_rect(&transform);
    let position_response = collider.position_response(&camera_rect);
    let position_response = Rect {
        min: position_response.min + collider.size,
        max: position_response.max - collider.size,
    };

    respond_to_horizontal_collision(
        &mut transform,
        &mut velocity,
        &player_rect,
        &camera_rect,
        &position_response,
    );
    respond_to_vertical_collision(
        &mut transform,
        &mut velocity,
        &player_rect,
        &camera_rect,
        &position_response,
    );
    if player_rect.min.y < camera_rect.min.y {
        commands.entity(player).remove::<Airborne>();
    }
}

pub fn coyote_jump(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, Option<&mut CoyoteJump>), With<Player>>,
) {
    let (player, coyote_jump) = query.single_mut();
    if coyote_jump.is_none() {
        return;
    }
    if coyote_jump.unwrap().0.tick(time.delta()).finished() {
        commands.entity(player).insert(Airborne);
        commands.entity(player).remove::<CoyoteJump>();
    }
}

pub fn reset_coyote_jump(mut commands: Commands, mut removed: RemovedComponents<Airborne>) {
    for entity in removed.iter() {
        commands
            .entity(entity)
            .insert(CoyoteJump(Timer::from_seconds(0.08, TimerMode::Once)));
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

fn is_colliding(lhs: &Rect, rhs: &Rect) -> bool {
    lhs.max.x > rhs.min.x && lhs.min.x < rhs.max.x && lhs.max.y > rhs.min.y && lhs.min.y < rhs.max.y
}

fn respond_to_horizontal_collision(
    transform: &mut Transform,
    velocity: &mut Velocity,
    rect: &Rect,
    other: &Rect,
    position_response: &Rect,
) {
    if rect.min.x < other.min.x {
        velocity.value.x = 0.0;
        transform.translation.x = position_response.min.x;
    } else if rect.max.x > other.max.x {
        velocity.value.x = 0.0;
        transform.translation.x = position_response.max.x;
    }
}

fn respond_to_vertical_collision(
    transform: &mut Transform,
    velocity: &mut Velocity,
    rect: &Rect,
    other: &Rect,
    position_response: &Rect,
) {
    if rect.min.y < other.min.y {
        velocity.value.y = 0.0;
        transform.translation.y = position_response.min.y;
    } else if rect.max.y > other.max.y {
        velocity.value.y = 0.0;
        transform.translation.y = position_response.max.y;
    }
}
