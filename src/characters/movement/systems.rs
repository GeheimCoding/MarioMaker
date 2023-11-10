use crate::characters::components::{Character, CollisionResponse};
use crate::characters::events::GroundedEvent;
use crate::characters::player::movement::components::{Airborne, JumpTimer};
use crate::characters::systems::is_colliding;
use crate::components::{Collider, MainCamera, Velocity};
use crate::world::components::Block;
use bevy::prelude::*;

pub fn horizontal_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Character>>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.value.x * time.delta_seconds();
    }
}

pub fn horizontal_block_collision_response(
    mut character_query: Query<
        (&Collider, &CollisionResponse, &mut Transform, &mut Velocity),
        With<Character>,
    >,
    block_query: Query<(&Collider, &Transform), (With<Block>, Without<Character>)>,
) {
    for (character_collider, collision_response, mut character_transform, mut velocity) in
        character_query.iter_mut()
    {
        for (block_collider, block_transform) in block_query.iter() {
            let character_rect = character_collider.get_rect(&character_transform);
            let block_rect = block_collider.get_rect(block_transform);

            if is_colliding(&character_rect, &block_rect) {
                let position_response = character_collider.position_response(&block_rect);
                respond_to_horizontal_collision(
                    &mut character_transform,
                    &mut velocity,
                    &character_rect,
                    &block_rect,
                    &position_response,
                    collision_response,
                );
            }
        }
    }
}

pub fn vertical_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Character>>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.y += velocity.value.y * time.delta_seconds();
    }
}

pub fn vertical_block_collision_response(
    mut commands: Commands,
    mut grounded_event: EventWriter<GroundedEvent>,
    mut character_query: Query<
        (
            Entity,
            &Collider,
            &CollisionResponse,
            &mut Transform,
            &mut Velocity,
        ),
        With<Character>,
    >,
    block_query: Query<(&Collider, &Transform), (With<Block>, Without<Character>)>,
) {
    for (
        character,
        character_collider,
        collision_response,
        mut character_transform,
        mut velocity,
    ) in character_query.iter_mut()
    {
        for (block_collider, block_transform) in block_query.iter() {
            let character_rect = character_collider.get_rect(&character_transform);
            let block_rect = block_collider.get_rect(block_transform);

            if is_colliding(&character_rect, &block_rect) {
                let position_response = character_collider.position_response(&block_rect);
                respond_to_vertical_collision(
                    &mut character_transform,
                    &mut velocity,
                    &character_rect,
                    &block_rect,
                    &position_response,
                    collision_response,
                );
                if character_rect.max.y > block_rect.max.y {
                    commands.entity(character).remove::<Airborne>();
                    grounded_event.send(GroundedEvent(character));
                } else {
                    commands.entity(character).remove::<JumpTimer>();
                }
            }
        }
    }
}

pub fn confine_in_window(
    mut commands: Commands,
    mut grounded_event: EventWriter<GroundedEvent>,
    mut character_query: Query<
        (
            Entity,
            &Collider,
            &CollisionResponse,
            &mut Transform,
            &mut Velocity,
        ),
        With<Character>,
    >,
    camera_query: Query<
        (&OrthographicProjection, &Transform),
        (With<MainCamera>, Without<Character>),
    >,
) {
    for (character, collider, collision_response, mut character_transform, mut velocity) in
        character_query.iter_mut()
    {
        let (projection, camera_transform) = camera_query.single();
        let camera_rect = Rect {
            min: projection.area.min + camera_transform.translation.truncate(),
            max: projection.area.max + camera_transform.translation.truncate(),
        };
        let character_rect = collider.get_rect(&character_transform);
        let position_response = collider.position_response(&camera_rect);
        let position_response = Rect {
            min: position_response.min + collider.size,
            max: position_response.max - collider.size,
        };
        let collision_response = CollisionResponse {
            velocity: collision_response.velocity * -1.0,
        };

        respond_to_horizontal_collision(
            &mut character_transform,
            &mut velocity,
            &character_rect,
            &camera_rect,
            &position_response,
            &collision_response,
        );
        respond_to_vertical_collision(
            &mut character_transform,
            &mut velocity,
            &character_rect,
            &camera_rect,
            &position_response,
            &collision_response,
        );
        if character_rect.min.y < camera_rect.min.y {
            commands.entity(character).remove::<Airborne>();
            grounded_event.send(GroundedEvent(character));
        } else if character_rect.max.y > camera_rect.max.y {
            commands.entity(character).remove::<JumpTimer>();
        }
    }
}

fn respond_to_horizontal_collision(
    transform: &mut Transform,
    velocity: &mut Velocity,
    rect: &Rect,
    other: &Rect,
    position_response: &Rect,
    collision_response: &CollisionResponse,
) {
    if rect.min.x < other.min.x {
        velocity.value.x = -collision_response.velocity.x;
        transform.translation.x = position_response.min.x;
    } else if rect.max.x > other.max.x {
        velocity.value.x = collision_response.velocity.x;
        transform.translation.x = position_response.max.x;
    }
}

fn respond_to_vertical_collision(
    transform: &mut Transform,
    velocity: &mut Velocity,
    rect: &Rect,
    other: &Rect,
    position_response: &Rect,
    collision_response: &CollisionResponse,
) {
    if rect.min.y < other.min.y {
        velocity.value.y = -collision_response.velocity.y;
        transform.translation.y = position_response.min.y;
    } else if rect.max.y > other.max.y {
        velocity.value.y = collision_response.velocity.y;
        transform.translation.y = position_response.max.y;
    }
}
