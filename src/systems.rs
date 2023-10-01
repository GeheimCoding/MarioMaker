use crate::components::{Acceleration, Animation, Direction, Velocity};
use bevy::prelude::*;
use std::fmt::Debug;

pub fn error_handler<E: Debug>(In(result): In<Result<(), E>>) {
    if let Err(err) = result {
        println!("encountered an error {:?}", err);
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.1,
            ..Camera2dBundle::default().projection
        },
        ..default()
    });
}

pub fn animate(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &mut TextureAtlasSprite, Option<&Direction>)>,
) {
    for (mut animation, mut sprite, direction) in query.iter_mut() {
        if animation.timer.tick(time.delta()).just_finished() {
            animation.frame_index = (animation.frame_index + 1) % animation.frames.len();
            sprite.index = animation.frames[animation.frame_index];
        }
        if let Some(direction) = direction {
            sprite.flip_x = direction == &Direction::Left;
        }
    }
}

pub fn apply_friction(time: Res<Time>, mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut velocity, acceleration) in query.iter_mut() {
        let change = acceleration.0 * time.delta_seconds();
        if velocity.value > 0.0 {
            velocity.value = (velocity.value - change).clamp(0.0, velocity.value);
        } else if velocity.value < 0.0 {
            velocity.value = (velocity.value + change).clamp(velocity.value, 0.0);
        }
    }
}
