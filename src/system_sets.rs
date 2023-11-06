use bevy::prelude::*;

pub struct SystemSetPlugin;

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum UpdateSet {
    HorizontalMovementActions,
    HorizontalMovement,
    VerticalMovementActions,
    VerticalMovement,
    HorizontalConfinement,
    VerticalConfinement,
    ChangeDetection,
}

impl Plugin for SystemSetPlugin {
    fn build(&self, app: &mut App) {
        use crate::system_sets::UpdateSet::*;
        app.configure_sets(Update, HorizontalMovementActions.before(HorizontalMovement))
            .configure_sets(Update, HorizontalMovement.before(HorizontalConfinement))
            .configure_sets(Update, VerticalMovementActions.before(VerticalMovement))
            .configure_sets(Update, VerticalMovement.before(VerticalConfinement))
            .configure_sets(
                Update,
                HorizontalConfinement.before(VerticalMovementActions),
            )
            .configure_sets(Update, HorizontalConfinement.before(ChangeDetection))
            .configure_sets(Update, VerticalConfinement.before(ChangeDetection));
    }
}
