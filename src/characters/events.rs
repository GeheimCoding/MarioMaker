use crate::components::Direction;
use bevy::prelude::*;

#[derive(Event)]
pub struct Grounded(pub Entity);

#[derive(Event)]
pub struct JumpedOn(pub Entity);

#[derive(Event)]
pub struct Kicked {
    pub entity: Entity,
    pub direction: Direction,
}
