use bevy::prelude::*;

#[derive(Component, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Component)]
pub struct Velocity {
    pub value: f32,
    pub max: f32,
}

impl Velocity {
    pub fn with_max(max: f32) -> Self {
        Velocity { value: 0.0, max }
    }
}

#[derive(Component)]
pub struct Acceleration(pub f32);

#[derive(Clone, Component)]
pub struct Animation {
    pub timer: Timer,
    pub frames: Vec<usize>,
    pub frame_index: usize,
}
