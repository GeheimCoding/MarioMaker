use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
pub enum State {
    #[default]
    Idle,
    Walking,
}
