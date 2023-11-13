use crate::editor::ui::UiPlugin;
use bevy::prelude::*;

mod ui;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiPlugin);
    }
}
