use bevy::prelude::*;

#[derive(Component, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Component)]
pub struct Velocity {
    pub value: Vec2,
    pub max: Vec2,
}

impl Velocity {
    pub fn with_max(max: Vec2) -> Self {
        Velocity {
            value: Vec2::ZERO,
            max,
        }
    }
}

#[derive(Clone, Component)]
pub struct Animation {
    pub timer: Timer,
    pub frames: Vec<usize>,
    pub frame_index: usize,
}
