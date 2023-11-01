use bevy::prelude::*;

use crate::{physics::components::{Collider, Velocity}, player::components::Player, EntityTookDamage};

use super::components::Enemy;

pub fn enemy_follow_player(
    mut player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(&Enemy, &Transform, &mut Velocity)>,
) {
    if let Ok(player_transform) = player_query.get_single_mut() {
        for (enemy, enemy_transform, mut enemy_velocity) in enemy_query.iter_mut() {
            let direction =
                (player_transform.translation - enemy_transform.translation).normalize();
            *enemy_velocity = Velocity::from_vec(direction * enemy.speed());
        }
    }
}

pub fn enemy_melee_player(
    player_query: Query<(&Transform, &Collider, Entity), With<Player>>,
    enemy_query: Query<(&Enemy, &Transform, &Collider, Entity)>,
    mut entity_took_damage_events: EventWriter<EntityTookDamage>,
) {
    if let Ok((player_transform, player_collider, player_entity)) = player_query.get_single() {
        for (enemy, enemy_transform, enemy_collider, enemy_entity) in enemy_query.iter() {
            if enemy_collider.is_colliding(enemy_transform, player_collider, player_transform) {
                entity_took_damage_events
                    .send(EntityTookDamage::new(player_entity, enemy.melee_damage()));
            }
        }
    }
}
