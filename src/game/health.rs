use bevy::{
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle},
    transform::commands,
};

use super::{GameSet, EntityTookDamage};

#[derive(Component, Debug)]
pub struct Health {
    health: i32,
    max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self { health: max, max }
    }

    pub fn damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn max(&self) -> i32 {
        self.max
    }

    pub fn health_percentage(&self) -> f32 {
        self.health as f32 / self.max as f32
    }

    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }
}

#[derive(Component, Debug)]
pub struct HealthBar {
    health_entity: Entity,
    width: f32,
}

impl HealthBar {
    pub fn new(health_entity: Entity, width: f32) -> Self {
        Self {
            health_entity,
            width,
        }
    }

    pub fn health_entity(&self) -> Entity {
        self.health_entity
    }

    pub fn max_width(&self) -> f32 {
        self.width
    }
}

#[derive(Bundle)]
struct HealthBarBundle<M: Material2d> {
    health_bar: HealthBar,
    material_mesh: MaterialMesh2dBundle<M>,
}

pub fn spawn_health_bar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    health_bar: HealthBar,
) -> Entity {
    let parent = health_bar.health_entity();

    let health_bar_entity = commands
        .spawn(HealthBarBundle {
            health_bar,
            material_mesh: MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad::new(Vec2::new(24., 2.))))
                    .into(),
                transform: Transform::default().with_translation(Vec3::new(0., -20., 0.)),
                material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                ..default()
            },
        })
        .id();

    commands.entity(parent).add_child(health_bar_entity);

    health_bar_entity
}

pub fn update_healthbar(
    mut meshes: ResMut<Assets<Mesh>>,
    bar_query: Query<(&Mesh2dHandle, &HealthBar)>,
    entity_query: Query<&Health, Changed<Health>>,
) {
    for (mesh_handle, health_bar) in bar_query.iter() {
        if let Ok(health) = entity_query.get(health_bar.health_entity()) {
            println!("update_healthbar");
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

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_healthbar).in_set(GameSet::Ui));
        app.add_systems(Update, (take_damage).in_set(GameSet::ResolveDamage));
    }
}
