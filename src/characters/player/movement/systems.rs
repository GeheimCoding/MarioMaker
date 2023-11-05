use crate::characters::components::{Grabbed, Hurting, Jumpable};
use crate::characters::events::{GroundedEvent, JumpedOnEvent};
use crate::characters::player::components::{Player, State};
use crate::characters::player::movement::components::{
    Acceleration, Airborne, CoyoteJump, JumpBuffer, JumpTimer,
};
use crate::characters::systems::is_colliding;
use crate::components::{Collider, Direction, Velocity};
use crate::world::components::Block;
use bevy::prelude::KeyCode::{Down, S};
use bevy::prelude::*;

pub fn run(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut grounded_event: EventReader<GroundedEvent>,
    mut query: Query<(Entity, &mut Direction, &mut Velocity, &Acceleration, &State), With<Player>>,
) {
    let (player, mut player_direction, mut player_velocity, acceleration, state) =
        query.single_mut();
    let grounded = grounded_event.iter().any(|event| event.0 == player);
    let crouching = *state == State::Grouching;
    let direction = get_horizontal_direction(keyboard_input);
    let is_moving = direction != 0.0 && !(crouching && grounded);
    let max_velocity = player_velocity.max.x;
    let velocity = &mut player_velocity.value.x;
    let acceleration = acceleration.0 * time.delta_seconds();

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
            Option<&mut JumpTimer>,
        ),
        With<Player>,
    >,
) {
    let (entity, mut velocity, airborne, jump_buffer, jump_timer) = query.single_mut();
    let mut jump = |commands: &mut Commands| {
        velocity.value.y = 200.0;
        commands.entity(entity).insert(Airborne);
        commands
            .entity(entity)
            .insert(JumpTimer(Timer::from_seconds(0.2, TimerMode::Once)));
    };

    if keyboard_input.just_pressed(KeyCode::Space) {
        if airborne.is_some() {
            commands
                .entity(entity)
                .insert(JumpBuffer(Timer::from_seconds(0.05, TimerMode::Once)));
        } else {
            jump(&mut commands);
        }
    }
    if let Some(mut jump_timer) = jump_timer {
        if keyboard_input.pressed(KeyCode::Space) {
            jump_timer.0.tick(time.delta());
        } else {
            commands.entity(entity).remove::<JumpTimer>();
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

pub fn crouch(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Collider, &mut State, &Transform), With<Player>>,
    block_query: Query<(&Collider, &Transform), (With<Block>, Without<Player>)>,
) {
    let (mut player_collider, mut state, player_transform) = player_query.single_mut();
    let down_pressed = keyboard_input.any_pressed(vec![S, Down]);

    if *state != State::Grouching && down_pressed {
        *state = State::Grouching;
        player_collider.size.y = 14.0;
        player_collider.offset.y = -9.0;
    } else if *state == State::Grouching && !down_pressed {
        let mut colliding = false;
        let mut updated_player_collider = *player_collider;
        updated_player_collider.size.y = 20.0;
        updated_player_collider.offset.y = -6.0;

        for (block_collider, block_transform) in block_query.iter() {
            let player_rect = updated_player_collider.get_rect(&player_transform);
            let block_rect = block_collider.get_rect(block_transform);

            if is_colliding(&player_rect, &block_rect) {
                colliding = true;
                break;
            }
        }
        if !colliding {
            *state = State::Idle;
            *player_collider = updated_player_collider;
        }
    }
}

pub fn gaze(
    keyboard_input: Res<Input<KeyCode>>,
    mut grounded_event: EventReader<GroundedEvent>,
    mut query: Query<(Entity, &mut State, &Velocity), With<Player>>,
) {
    let (player, mut state, velocity) = query.single_mut();
    let grounded = grounded_event.iter().any(|event| event.0 == player);

    if velocity.value.x == 0.0 && grounded {
        if keyboard_input.any_pressed(vec![KeyCode::W, KeyCode::Up]) {
            *state = State::Gazing;
        } else if *state == State::Gazing {
            *state = State::Idle;
        }
    }
}

pub fn jump_on(
    mut commands: Commands,
    mut jumped_on_event: EventWriter<JumpedOnEvent>,
    mut player_query: Query<(Entity, &Collider, &mut Transform, &mut Velocity), With<Player>>,
    mut enemy_query: Query<
        (Entity, &Collider, &Transform),
        (With<Jumpable>, Without<Player>, Without<Grabbed>),
    >,
) {
    let (player, player_collider, mut player_transform, mut velocity) = player_query.single_mut();
    for (enemy, enemy_collider, enemy_transform) in enemy_query.iter_mut() {
        let player_rect = player_collider.get_rect(&player_transform);
        let enemy_rect = enemy_collider.get_rect(enemy_transform);

        if is_colliding(&player_rect, &enemy_rect) {
            let position_response = player_collider.position_response(&enemy_rect);
            if velocity.value.y < 0.0 {
                player_transform.translation.y = position_response.max.y;
                velocity.value.y = 200.0;
                commands.entity(player).insert(Airborne);
                commands
                    .entity(player)
                    .insert(JumpTimer(Timer::from_seconds(0.2, TimerMode::Once)));
                jumped_on_event.send(JumpedOnEvent(enemy));
            }
        }
    }
}

pub fn take_damage(
    mut player_query: Query<(&Collider, &mut Transform), With<Player>>,
    mut enemy_query: Query<(&Collider, &Transform), (With<Hurting>, Without<Player>)>,
) {
    let (player_collider, mut player_transform) = player_query.single_mut();
    for (enemy_collider, enemy_transform) in enemy_query.iter_mut() {
        let player_rect = player_collider.get_rect(&player_transform);
        let enemy_rect = enemy_collider.get_rect(enemy_transform);

        if is_colliding(&player_rect, &enemy_rect) {
            let distance = enemy_rect.max.y - player_rect.min.y;
            if distance > 4.0 {
                player_transform.translation = Vec3::ZERO;
            }
        }
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
