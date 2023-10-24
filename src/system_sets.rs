use bevy::prelude::*;

pub struct SystemSetPlugin;

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum UpdateSet {
    HorizontalMovement,
    VerticalMovement,
    HorizontalConfinement,
    VerticalConfinement,
    ChangeDetection,
}

impl Plugin for SystemSetPlugin {
    fn build(&self, app: &mut App) {
        use crate::system_sets::UpdateSet::*;
        app.configure_set(Update, HorizontalMovement.before(HorizontalConfinement))
            .configure_set(Update, VerticalMovement.before(VerticalConfinement))
            .configure_set(Update, HorizontalConfinement.before(VerticalMovement))
            .configure_set(Update, HorizontalConfinement.before(ChangeDetection))
            .configure_set(Update, VerticalConfinement.before(ChangeDetection));
    }
}
