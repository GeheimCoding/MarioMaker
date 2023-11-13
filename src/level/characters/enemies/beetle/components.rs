use bevy::prelude::*;

#[derive(Component)]
pub struct Beetle;

#[derive(Clone, Component, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum State {
    #[default]
    IdleAlive,
    IdleDead,
    Walking,
    Rolling,
}

#[derive(Component)]
pub struct KickTimer(pub Timer);
