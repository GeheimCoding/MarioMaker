use bevy::prelude::*;

#[derive(Component, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
