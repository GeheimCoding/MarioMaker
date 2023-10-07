use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Clone, Component, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum State {
    #[default]
    Idle,
    Walking,
    Jumping,
}
