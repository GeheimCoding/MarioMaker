use crate::characters::enemies::beetle::BeetlePlugin;
use bevy::prelude::*;

mod beetle;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BeetlePlugin);
    }
}
