use bevy::prelude::*;

#[derive(Event)]
pub struct Grounded(pub Entity);

#[derive(Event)]
pub struct JumpedOn(pub Entity);
