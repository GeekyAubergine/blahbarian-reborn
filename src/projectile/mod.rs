use bevy::prelude::*;

use systems::projectile_hurt_entity;

use crate::{GameSet, physics::components::{Velocity, Collider}, EnitityAllegence};

use self::components::Projectile;

pub mod components;
pub mod systems;

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub transform: Transform,
    pub velocity: Velocity,
    pub collider: Collider,
    pub projectile: Projectile,
    pub allegence: EnitityAllegence,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (projectile_hurt_entity).in_set(GameSet::DealDamage));
    }
}
