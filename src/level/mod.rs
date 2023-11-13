use crate::level::characters::CharacterPlugin;
use bevy::prelude::*;

pub mod characters;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CharacterPlugin);
    }
}
