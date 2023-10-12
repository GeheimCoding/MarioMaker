use bevy::prelude::*;

#[derive(Default, Deref, Resource)]
pub struct MousePosition(pub Vec2);
