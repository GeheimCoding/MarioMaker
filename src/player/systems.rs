use crate::components::{Animation, Direction, Gravity, Velocity, MIN_ANIMATION_DURATION};
use crate::content_manager::{TextureData, Textures};
use crate::player::components::{Player, State};
use crate::player::movement::components::{Acceleration, Jumping};
use crate::player::resources::{Animations, Texture};
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
            texture: Texture::Mario,
            path: "textures/mario.png".to_owned(),
            tile_size: Vec2::new(16.0, 24.0),
            columns: 3,
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
        // this prevents being able to jump when spawning in mid-air
        Jumping,
    ));
}

pub fn change_state(
    mut query: Query<(&mut State, &Velocity, Option<&Jumping>), (With<Player>, Changed<Velocity>)>,
) {
    for (mut state, velocity, jumping) in query.iter_mut() {
        if jumping.is_some() {
            *state = State::Jumping;
        } else if velocity.value.x.abs() > 0.0 {
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
