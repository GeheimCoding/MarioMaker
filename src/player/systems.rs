use crate::components::{
    Animation, Collider, Direction, Gravity, Velocity, MIN_ANIMATION_DURATION,
};
use crate::content_manager::{TextureData, Textures};
use crate::player::components::{Player, State};
use crate::player::movement::components::{Acceleration, Airborne, CoyoteJump};
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
        Collider {
            size: Vec2::new(14.0, 20.0),
            offset: Vec2::new(0.0, -1.0),
        },
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
        // this prevents being able to jump right away when spawning in mid-air
        Airborne,
    ));
}

pub fn handle_velocity_change(
    mut query: Query<
        (
            &mut State,
            &Velocity,
            Option<&Airborne>,
            Option<&mut CoyoteJump>,
        ),
        (With<Player>, Changed<Velocity>),
    >,
) {
    if query.is_empty() {
        return;
    }
    let (mut state, velocity, airborne, coyote_jump) = query.single_mut();
    if airborne.is_some() {
        *state = State::Airborne;
    } else if velocity.value.x.abs() > 0.0 {
        *state = State::Walking;
    } else {
        *state = State::Idle;
    }
    if velocity.value.y == 0.0 {
        if let Some(mut coyote_jump) = coyote_jump {
            coyote_jump.0.reset();
        }
    }
}

pub fn handle_state_change(
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
