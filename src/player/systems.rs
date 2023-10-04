use crate::components::{Animation, Direction, Gravity, Velocity};
use crate::player::components::{Acceleration, Jumping, Player, State};
use crate::player::resources::{Animations, Texture, Textures, MIN_ANIMATION_DURATION};
use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;

pub fn spawn(mut commands: Commands, textures: Res<Textures>, animations: Res<Animations>) {
    commands.spawn((
        Player,
        Velocity::with_max(Vec2::new(100.0, 400.0)),
        Acceleration(350.0),
        Gravity(1200.0),
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
        let reset_sprite_index =
            animation.timer.elapsed_secs() >= MIN_ANIMATION_DURATION || *last_state == State::Idle;

        *last_state = state;
        *animation = animations.get(&state);
        if reset_sprite_index {
            sprite.index = animation.frames[animation.frame_index]
        }
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
        let factor = if direction.signum() == velocity.signum() {
            1.0
        } else {
            3.0
        };
        (velocity + direction * acceleration * factor).clamp(-max_velocity, max_velocity)
    } else {
        apply_friction(velocity, acceleration * 1.2)
    };
    transform.translation.x += velocity * time.delta_seconds();

    Ok(())
}

pub fn vertical_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Velocity), With<Player>>,
) -> Result<(), QuerySingleError> {
    let (entity, mut transform, mut velocity) = query.get_single_mut()?;
    let threshold = -20.0;

    transform.translation.y += velocity.value.y * time.delta_seconds();
    if transform.translation.y < threshold {
        velocity.value.y = 0.0;
        transform.translation.y = threshold;
        commands.entity(entity).remove::<Jumping>();
    }
    Ok(())
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
        let (entity, mut velocity) = query.get_single_mut().unwrap();
        commands.entity(entity).insert(Jumping);
        velocity.value.y = 250.0;
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
