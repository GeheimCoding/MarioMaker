use crate::characters::components::Kickable;
use crate::characters::events::Kicked;
use crate::characters::player::components::Player;
use crate::components::Collider;
use crate::components::Direction;
use bevy::prelude::*;

pub fn is_colliding(lhs: &Rect, rhs: &Rect) -> bool {
    lhs.max.x > rhs.min.x && lhs.min.x < rhs.max.x && lhs.max.y > rhs.min.y && lhs.min.y < rhs.max.y
}

pub fn kick(
    mut kicked_event: EventWriter<Kicked>,
    mut player_query: Query<(&Collider, &Transform), With<Player>>,
    mut enemy_query: Query<(Entity, &Collider, &Transform), (With<Kickable>, Without<Player>)>,
) {
    let (player_collider, player_transform) = player_query.single_mut();
    for (enemy, enemy_collider, enemy_transform) in enemy_query.iter_mut() {
        let player_rect = player_collider.get_rect(&player_transform);

        let enemy_rect = enemy_collider.get_rect(enemy_transform);

        if is_colliding(&player_rect, &enemy_rect) {
            let direction = if player_rect.min.x < enemy_rect.min.x {
                Direction::Right
            } else {
                Direction::Left
            };
            kicked_event.send(Kicked {
                entity: enemy,
                direction,
            });
        }
    }
}
