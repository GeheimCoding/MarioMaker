use crate::components::{Acceleration, Animation, Direction, Velocity};
use crate::player::components::{Player, State};
use crate::player::resources::{Animations, Texture, Textures};
use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;

pub fn spawn(mut commands: Commands, textures: Res<Textures>, animations: Res<Animations>) {
    commands.spawn((
        Player,
        Velocity::with_max(80.0),
        Acceleration(300.0),
        State::Idle,
        Direction::Right,
        SpriteSheetBundle {
            texture_atlas: textures.get(&Texture::Mario),
            ..default()
        },
        animations.get(&State::Idle),
    ));
}

pub fn change_animation(
    animations: Res<Animations>,
    mut query: Query<
        (&mut TextureAtlasSprite, &mut Animation, &State),
        (With<Player>, Changed<State>),
    >,
    mut last_state: Local<State>,
) -> Result<(), QuerySingleError> {
    let (mut sprite, mut animation, &state) = query.get_single_mut()?;
    if *last_state == state {
        return Ok(());
    }

    *last_state = state;
    *animation = animations.get(&state);
    sprite.index = animation.frames[animation.frame_index];

    Ok(())
}

pub fn horizontal_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<
        (
            &mut Transform,
            &mut Direction,
            &mut State,
            &mut Velocity,
            &Acceleration,
        ),
        With<Player>,
    >,
) -> Result<(), QuerySingleError> {
    let (mut transform, mut player_direction, mut state, mut velocity, acceleration) =
        query.get_single_mut()?;
    let direction = get_x_direction(keyboard_input);

    if direction < 0.0 {
        *player_direction = Direction::Left;
        *state = State::Walking;
    } else if direction > 0.0 {
        *player_direction = Direction::Right;
        *state = State::Walking;
    } else {
        *state = State::Idle;
    }

    velocity.value = (velocity.value + direction * 2.0 * acceleration.0 * time.delta_seconds())
        .clamp(-velocity.max, velocity.max);
    transform.translation.x += velocity.value * time.delta_seconds();

    Ok(())
}

fn get_x_direction(keyboard_input: Res<Input<KeyCode>>) -> f32 {
    let mut direction = 0.0;
    if keyboard_input.any_pressed(vec![KeyCode::Left, KeyCode::A]) {
        direction -= 1.0;
    }
    if keyboard_input.any_pressed(vec![KeyCode::Right, KeyCode::D]) {
        direction += 1.0;
    }
    direction
}
