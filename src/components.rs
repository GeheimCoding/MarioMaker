use bevy::prelude::*;

#[derive(Component, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Component)]
pub struct Animation {
    pub timer: Timer,
    pub frames: Vec<usize>,
    pub current_frame: usize,
}
