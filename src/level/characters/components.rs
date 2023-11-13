use bevy::prelude::*;

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct CollisionResponse {
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct Jumpable;

#[derive(Component)]
pub struct Kickable;

#[derive(Component)]
pub struct Grabable;

#[derive(Component)]
pub struct Grabbing(pub Entity);

#[derive(Component)]
pub struct Grabbed;

#[derive(Component)]
pub struct Hurting;
