use bevy::prelude::*;

#[derive(Default, Deref, Resource)]
pub struct MousePosition(pub Vec2);

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum AppState {
    #[default]
    Editor,
    Level,
}
