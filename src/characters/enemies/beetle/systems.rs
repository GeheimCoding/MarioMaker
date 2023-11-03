use crate::characters::components::{
    Character, CollisionResponse, Grabable, Grabbed, Hurting, Jumpable, Kickable,
};
use crate::characters::enemies::beetle::components::{Beetle, KickTimer, State};
use crate::characters::enemies::beetle::resources::{Animations, Texture};
use crate::characters::events::{GrabbedEvent, JumpedOnEvent, KickedEvent};
use crate::components::{Animation, Collider, Direction, Gravity, Velocity};
use crate::content_manager::{TextureData, Textures};
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
            texture: Texture::Beetle,
            path: "textures/beetle.png".to_owned(),
            tile_size: Vec2::new(16.0, 16.0),
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
        Beetle,
        Hurting,
        Jumpable,
        Character,
        CollisionResponse {
            velocity: Vec2::new(50.0, 0.0),
        },
        State::Walking,
        Direction::Right,
        Collider {
            size: Vec2::splat(16.0),
            offset: Vec2::ZERO,
        },
        Velocity {
            value: Vec2::new(50.0, 0.0),
            max: Vec2::new(50.0, 400.0),
        },
        Gravity(1200.0),
        SpriteSheetBundle {
            texture_atlas: textures.get(&Texture::Beetle),
            transform: Transform::from_translation(Vec3::new(96.0, 0.0, 0.0)),
            ..default()
        },
        animations.get(&State::Walking),
    ));
}

pub fn handle_velocity_change(
    mut query: Query<(&mut Direction, &Velocity), (With<Beetle>, Changed<Velocity>)>,
) {
    for (mut direction, velocity) in query.iter_mut() {
        if velocity.value.x < 0.0 {
            *direction = Direction::Left;
        } else if velocity.value.x > 0.0 {
            *direction = Direction::Right;
        }
    }
}

pub fn die(
    mut commands: Commands,
    animations: Res<Animations>,
    mut jumped_on_event: EventReader<JumpedOnEvent>,
    mut query: Query<
        (
            Entity,
            &mut Animation,
            &mut Velocity,
            &mut CollisionResponse,
        ),
        (With<Beetle>, Without<Grabbed>),
    >,
) {
    for (beetle, mut animation, mut velocity, mut collision_response) in query.iter_mut() {
        if jumped_on_event.iter().any(|event| event.0 == beetle) {
            *animation = animations.get(&State::IdleDead);
            velocity.value.x = 0.0;
            collision_response.velocity = Vec2::ZERO;
            commands.entity(beetle).remove::<(Jumpable, Hurting)>();
            commands.entity(beetle).insert((Grabable, Kickable));
        }
    }
}

pub fn get_kicked(
    mut commands: Commands,
    animations: Res<Animations>,
    mut kicked_event: EventReader<KickedEvent>,
    mut grabbed_event: EventReader<GrabbedEvent>,
    mut query: Query<
        (
            Entity,
            &mut Animation,
            &mut Velocity,
            &mut CollisionResponse,
        ),
        (With<Beetle>, Without<Grabbed>),
    >,
) {
    let speed = 200.0;
    for (beetle, mut animation, mut velocity, mut collision_response) in query.iter_mut() {
        for event in kicked_event.iter() {
            if event.entity != beetle || grabbed_event.iter().any(|event| event.0 == beetle) {
                continue;
            }
            *animation = animations.get(&State::Rolling);
            velocity.value.x = if event.direction == Direction::Left {
                -speed
            } else {
                speed
            };
            collision_response.velocity.x = speed;
            commands.entity(beetle).remove::<(Kickable, Grabable)>();
            commands
                .entity(beetle)
                .insert(KickTimer(Timer::from_seconds(0.16, TimerMode::Once)));
        }
    }
}

pub fn reset_jumpable(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut KickTimer), (With<Beetle>, Without<Grabbed>)>,
) {
    for (beetle, mut kick_timer) in query.iter_mut() {
        if kick_timer.0.tick(time.delta()).just_finished() {
            commands.entity(beetle).remove::<KickTimer>();
            commands.entity(beetle).insert((Hurting, Jumpable));
        }
    }
}

pub fn get_grabbed(
    mut commands: Commands,
    animations: Res<Animations>,
    mut grabbed_event: EventReader<GrabbedEvent>,
    mut query: Query<
        (
            Entity,
            &mut Animation,
            &mut Velocity,
            &mut CollisionResponse,
        ),
        With<Beetle>,
    >,
    grabbed_query: Query<Entity, With<Grabbed>>,
) {
    if !grabbed_query.is_empty() {
        return;
    }
    for (beetle, mut animation, mut velocity, mut collision_response) in query.iter_mut() {
        for event in grabbed_event.iter() {
            if event.0 != beetle {
                continue;
            }
            *animation = animations.get(&State::IdleDead);
            velocity.value = Vec2::ZERO;
            collision_response.velocity = Vec2::ZERO;
            commands.entity(beetle).insert(Grabbed);
            return;
        }
    }
}
