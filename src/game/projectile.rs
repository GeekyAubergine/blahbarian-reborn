use bevy::prelude::*;

use super::{
    physics::{Collider, Velocity},
    EnitityAllegence, GameSet, EntityTookDamage,
};

#[derive(Component)]
pub struct Projectile {
    damage: i32,
}

impl Projectile {
    pub fn new(damage: i32) -> Self {
        Self { damage }
    }

    pub fn damage(&self) -> i32 {
        self.damage
    }
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub transform: Transform,
    pub velocity: Velocity,
    pub collider: Collider,
    pub projectile: Projectile,
    pub allegence: EnitityAllegence,
}

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


pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (projectile_hurt_entity).in_set(GameSet::DealDamage));
    }
}
