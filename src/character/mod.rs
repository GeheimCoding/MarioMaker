use crate::character::enemy::EnemyPlugin;
use crate::character::player::PlayerPlugin;
use bevy::prelude::*;

mod enemy;
pub mod player;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, EnemyPlugin));
    }
}
