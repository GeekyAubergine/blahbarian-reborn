use bevy::prelude::*;
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle};

use crate::{
    health::{
        components::{Health, HealthBar},
        spawn_health_bar,
    },
    physics::components::{Collider, Velocity},
    EnitityAllegence, EntityTookDamage, GameSet,
};

use self::{
    components::Enemy,
    systems::{enemy_follow_player, enemy_melee_player},
};

pub mod components;
pub mod systems;

mod sprites {
    use bevy_aseprite::aseprite;

    aseprite!(pub TableAnim, "table.aseprite");
}

#[derive(Bundle)]
pub struct EnemyBundle {
    collider: Collider,
    aesprite: AsepriteBundle,
    enemy: Enemy,
    allegence: EnitityAllegence,
    health: Health,
    velocity: Velocity,
}

pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let entity = commands
        .spawn(EnemyBundle {
            collider: Collider::circle(32.),
            aesprite: AsepriteBundle {
                aseprite: asset_server.load(sprites::TableAnim::PATH),
                animation: AsepriteAnimation::from(sprites::TableAnim::tags::IDLE),
                transform: Transform {
                    scale: Vec3::splat(2.),
                    translation: Vec3::new(0., 128., 0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            enemy: Enemy::Table {
                last_melee: 0.0,
                health_entity: Entity::PLACEHOLDER,
            },
            allegence: EnitityAllegence::Enemy,
            health: Health::new(100),
            velocity: Velocity::zero(),
        })
        .id();

    spawn_health_bar(commands, meshes, materials, HealthBar::new(entity, 24.));
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (enemy_follow_player).in_set(GameSet::Ai),
        );
        app.add_systems(
            Update,
            (enemy_melee_player).in_set(GameSet::DealDamage),
        );
    }
}
