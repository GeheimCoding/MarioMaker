use bevy::prelude::*;

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct CollisionResponse {
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct Jumpable;
