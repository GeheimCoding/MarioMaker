use crate::characters::components::{Character, CollisionResponse, Grabable, Grabbed, Kickable};
use crate::characters::events::{GrabbedEvent, KickedEvent};
use crate::characters::player::components::{Player, State};
use crate::characters::player::movement::components::{Acceleration, Airborne, CoyoteJump};
use crate::characters::player::resources::{Animations, Texture};
use crate::characters::systems::is_colliding;
use crate::components::{
    Animation, Collider, Direction, Gravity, Velocity, MIN_ANIMATION_DURATION,
};
use crate::content_manager::{TextureData, Textures};
use crate::world::components::TILE_SIZE;
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
            columns: 6,
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
        Character,
        CollisionResponse {
            velocity: Vec2::ZERO,
        },
        Collider {
            size: Vec2::new(14.0, 20.0),
            offset: Vec2::new(0.0, -1.0),
        },
        Velocity::with_max(Vec2::new(160.0, 400.0)),
        Acceleration(400.0),
        Gravity(1500.0),
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
    if velocity.value.y == 0.0 {
        if let Some(mut coyote_jump) = coyote_jump {
            coyote_jump.0.reset();
        }
    }
    if *state == State::Grouching {
        return;
    }
    if airborne.is_some() {
        *state = if velocity.value.y > 0.0 {
            State::Jumping
        } else {
            State::Falling
        };
    } else if velocity.value.x.abs() > 0.0 {
        *state = State::Running;
    } else if *state != State::Gazing {
        *state = State::Idle;
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

pub fn move_camera(
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();
    let right_edge = player_transform.translation.x + TILE_SIZE;

    if camera_transform.translation.x < right_edge {
        camera_transform.translation.x = right_edge;
    }
    camera_transform.translation.y = player_transform.translation.y.clamp(0.0, 100.0);
}

pub fn kick(
    mut kicked_event: EventWriter<KickedEvent>,
    mut grabbed_event: EventReader<GrabbedEvent>,
    mut player_query: Query<(&Collider, &Transform, &Velocity), With<Player>>,
    mut enemy_query: Query<
        (Entity, &Collider, &Transform),
        (With<Kickable>, Without<Player>, Without<Grabbed>),
    >,
) {
    let (player_collider, player_transform, velocity) = player_query.single_mut();
    for (enemy, enemy_collider, enemy_transform) in enemy_query.iter_mut() {
        if grabbed_event.iter().any(|event| event.0 == enemy) {
            continue;
        }
        let player_rect = player_collider.get_rect(&player_transform);
        let enemy_rect = enemy_collider.get_rect(enemy_transform);

        if is_colliding(&player_rect, &enemy_rect) {
            let direction = if player_rect.min.x < enemy_rect.min.x {
                Direction::Right
            } else {
                Direction::Left
            };
            kicked_event.send(KickedEvent {
                entity: enemy,
                direction,
                velocity: velocity.value,
            });
        }
    }
}

pub fn grab(
    keyboard_input: Res<Input<KeyCode>>,
    mut grabbed_event: EventWriter<GrabbedEvent>,
    mut player_query: Query<(&Collider, &Transform), With<Player>>,
    mut enemy_query: Query<
        (Entity, &Collider, &Transform),
        (With<Grabable>, Without<Player>, Without<Grabbed>),
    >,
) {
    if !keyboard_input.pressed(KeyCode::ShiftLeft) {
        return;
    }
    let (player_collider, player_transform) = player_query.single_mut();
    for (enemy, enemy_collider, enemy_transform) in enemy_query.iter_mut() {
        let player_rect = player_collider.get_rect(&player_transform);
        let enemy_rect = enemy_collider.get_rect(enemy_transform);

        if is_colliding(&player_rect, &enemy_rect) {
            grabbed_event.send(GrabbedEvent(enemy));
        }
    }
}

pub fn hold_item(
    player_query: Query<(&Transform, &Direction), With<Player>>,
    mut item_query: Query<&mut Transform, (With<Grabbed>, Without<Player>)>,
) {
    if item_query.is_empty() {
        return;
    }
    let (player_transform, direction) = player_query.single();
    let mut item_transform = item_query.single_mut();

    item_transform.translation = player_transform.translation;
    let offset = if *direction == Direction::Right {
        10.0
    } else {
        -10.0
    };
    item_transform.translation.x += offset;
}

pub fn kick_held_item(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut kicked_event: EventWriter<KickedEvent>,
    player_query: Query<(&Direction, &Velocity), With<Player>>,
    item_query: Query<Entity, With<Grabbed>>,
) {
    if item_query.is_empty() {
        return;
    }
    let (direction, velocity) = player_query.single();
    let item = item_query.single();
    let direction = if keyboard_input.any_pressed(vec![KeyCode::Down, KeyCode::S]) {
        Direction::Down
    } else if keyboard_input.any_pressed(vec![KeyCode::Up, KeyCode::W]) {
        Direction::Up
    } else {
        *direction
    };

    if keyboard_input.just_released(KeyCode::ShiftLeft) {
        commands
            .entity(item)
            .remove::<(Grabbed, Grabable, Kickable)>();
        kicked_event.send(KickedEvent {
            entity: item,
            direction,
            velocity: velocity.value,
        });
    }
}
