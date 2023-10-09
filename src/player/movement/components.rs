use bevy::prelude::*;

#[derive(Component)]
pub struct Acceleration(pub f32);

#[derive(Component)]
pub struct Airborne;

#[derive(Component)]
pub struct CoyoteJump(pub Timer);
