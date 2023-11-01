use bevy::{
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle},
    transform::commands,
};

use crate::GameSet;

use self::{
    components::{Health, HealthBar},
    systems::{take_damage, update_healthbar},
};

pub mod components;
pub mod systems;

#[derive(Component)]
pub struct HealthBarForegroundHandle(pub Mesh2dHandle);

#[derive(Component)]
pub struct HealthBarBackgroundHandle(pub Mesh2dHandle);

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

    commands
        .entity(parent)
        .add_child(health_bar_entity);

    health_bar_entity
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_healthbar).in_set(GameSet::Ui));
        app.add_systems(Update, (take_damage).in_set(GameSet::ResolveDamage));
    }
}
