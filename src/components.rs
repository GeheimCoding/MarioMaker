use bevy::prelude::*;

#[derive(Component, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Clone, Component)]
pub struct Animation {
    pub timer: Timer,
    pub frames: Vec<usize>,
    pub frame_index: usize,
}
