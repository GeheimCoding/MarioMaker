use crate::components::Direction;
use bevy::prelude::*;

#[derive(Event)]
pub struct GroundedEvent(pub Entity);

#[derive(Event)]
pub struct JumpedOnEvent(pub Entity);

#[derive(Event)]
pub struct KickedEvent {
    pub entity: Entity,
    pub direction: Direction,
    pub velocity: Vec2,
}

#[derive(Event)]
pub struct GrabbedEvent(pub Entity);
