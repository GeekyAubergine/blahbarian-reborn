use bevy::prelude::*;

use crate::game::{physics::components::Collider, EnitityAllegence, EntityTookDamage};

use super::components::Projectile;

pub fn projectile_hurt_entity(
    mut commands: Commands,
    projectile_query: Query<(
        Entity,
        &Projectile,
        &Transform,
        &Collider,
        &EnitityAllegence,
    )>,
    mut collidable_entity_query: Query<(&Collider, &Transform, &EnitityAllegence, Entity)>,
    mut entity_took_damage_events: EventWriter<EntityTookDamage>,
) {
    for (
        projectile_entity,
        projectile,
        projectile_transform,
        projectile_collider,
        protectile_allegence,
    ) in projectile_query.iter()
    {
        for (entity_collider, entity_tranform, entity_allegence, entity) in collidable_entity_query.iter_mut()
        {
            if entity_allegence == protectile_allegence {
                continue;
            }

            if entity_collider.is_colliding(
                entity_tranform,
                projectile_collider,
                projectile_transform,
            ) {
                dbg!("HIT");
                entity_took_damage_events.send(EntityTookDamage::new(entity, projectile.damage()));
                commands.entity(projectile_entity).despawn();
            }
        }
    }
}
