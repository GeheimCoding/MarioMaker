use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Clone, Component, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum State {
    #[default]
    Idle,
    Walking,
}

#[derive(Component)]
pub struct Acceleration(pub f32);

#[derive(Component)]
pub struct Jumping;
