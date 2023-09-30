use crate::components::{Animation, Direction};
use crate::player::components::{Player, State};
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
        State::Idle,
        Direction::Right,
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..default()
        },
        get_animation(State::Idle),
    ));
}

pub fn change_animation(
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
    *animation = get_animation(state);
    sprite.index = animation.frames[animation.frame_index];

    Ok(())
}

pub fn horizontal_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Direction, &mut State, &Player)>,
) -> Result<(), QuerySingleError> {
    let (mut transform, mut player_direction, mut state, player) = query.get_single_mut()?;
    let direction = get_direction(keyboard_input);

    if direction.x < 0.0 {
        *player_direction = Direction::Left;
        *state = State::Walking;
    } else if direction.x > 0.0 {
        *player_direction = Direction::Right;
        *state = State::Walking;
    } else {
        *state = State::Idle;
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

fn get_animation(state: State) -> Animation {
    match state {
        State::Idle => Animation {
            timer: default(),
            frames: vec![0],
            frame_index: 0,
        },
        State::Walking => Animation {
            timer: Timer::from_seconds(0.15, TimerMode::Repeating),
            frames: vec![0, 1],
            frame_index: 1,
        },
    }
}
