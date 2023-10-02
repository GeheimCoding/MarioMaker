use crate::components::{Animation, Direction, Velocity};
use crate::player::components::{Acceleration, Player, State};
use crate::player::resources::{Animations, Texture, Textures};
use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;

pub fn spawn(mut commands: Commands, textures: Res<Textures>, animations: Res<Animations>) {
    commands.spawn((
        Player,
        Velocity::with_max(Vec2::new(80.0, 0.0)),
        Acceleration(350.0),
        State::Idle,
        Direction::Right,
        SpriteSheetBundle {
            texture_atlas: textures.get(&Texture::Mario),
            ..default()
        },
        animations.get(&State::Idle),
    ));
}

pub fn change_state(mut query: Query<(&mut State, &Velocity), (With<Player>, Changed<Velocity>)>) {
    for (mut state, velocity) in query.iter_mut() {
        if velocity.value.x.abs() > 0.0 {
            *state = State::Walking;
        } else {
            *state = State::Idle;
        }
    }
}

pub fn change_animation(
    animations: Res<Animations>,
    mut query: Query<
        (&mut TextureAtlasSprite, &mut Animation, &State),
        (With<Player>, Changed<State>),
    >,
    mut last_state: Local<State>,
) {
    for (mut sprite, mut animation, &state) in query.iter_mut() {
        if *last_state == state {
            return;
        }

        *last_state = state;
        *animation = animations.get(&state);
        sprite.index = animation.frames[animation.frame_index];
    }
}

pub fn horizontal_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Direction, &mut Velocity, &Acceleration), With<Player>>,
) -> Result<(), QuerySingleError> {
    let (mut transform, mut player_direction, mut player_velocity, acceleration) =
        query.get_single_mut()?;
    let direction = get_horizontal_direction(keyboard_input);
    let is_moving = direction != 0.0;
    let max_velocity = player_velocity.max.x;
    let acceleration = acceleration.0 * time.delta_seconds();
    let velocity = player_velocity.value.x;

    if velocity == 0.0 && !is_moving {
        return Ok(());
    }

    if direction < 0.0 {
        *player_direction = Direction::Left;
    } else if direction > 0.0 {
        *player_direction = Direction::Right;
    }

    player_velocity.value.x = if is_moving {
        let factor = if direction.abs() > 0.0 && direction.signum() - velocity.signum() != 0.0 {
            1.5
        } else {
            1.0
        };
        (velocity + direction * acceleration * factor).clamp(-max_velocity, max_velocity)
    } else {
        apply_friction(velocity, acceleration)
    };
    transform.translation.x += velocity * time.delta_seconds();

    Ok(())
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
