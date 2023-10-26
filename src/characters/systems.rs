use crate::characters::components::Character;
use crate::components::Velocity;
use bevy::prelude::*;

pub fn vertical_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Character>>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.y += velocity.value.y * time.delta_seconds();
    }
}
