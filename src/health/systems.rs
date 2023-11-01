use bevy::{prelude::*, sprite::Mesh2dHandle};

use crate::EntityTookDamage;

use super::components::{Health, HealthBar};

pub fn update_healthbar(
    mut meshes: ResMut<Assets<Mesh>>,
    bar_query: Query<(&Mesh2dHandle, &HealthBar)>,
    entity_query: Query<&Health>,
) {
    for (mesh_handle, health_bar) in bar_query.iter() {
        if let Ok(health) = entity_query.get(health_bar.health_entity()) {
            let new_width = health_bar.max_width() * health.health_percentage();

            let quad = Mesh::from(shape::Quad::new(Vec2::new(new_width, 2.)));

            let handle: Handle<Mesh> = mesh_handle.0.clone();

            let _ = meshes.set(handle, quad);
        }
    }
}

pub fn take_damage(
    mut commands: Commands,
    mut entity_query: Query<(Entity, &mut Health)>,
    mut entity_took_damage_events: EventReader<EntityTookDamage>,
) {
    for event in entity_took_damage_events.iter() {
        if let Ok((entity, mut health)) = entity_query.get_mut(event.entity) {
            health.damage(event.damage);

            if health.is_dead() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
