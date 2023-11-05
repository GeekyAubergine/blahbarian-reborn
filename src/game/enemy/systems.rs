use bevy::prelude::*;
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle};
use bevy_prng::ChaCha8Rng;
use bevy_rand::resource::GlobalEntropy;
use rand_core::RngCore;

use crate::game::{physics::components::{Collider, Velocity}, player::components::Player, EnitityAllegence, health::{components::{Health, HealthBar}, spawn_health_bar}, EntityTookDamage};

use super::{components::Enemy, sprites, EnemyBundle, EnemySpawnConfig};

pub fn spawn_enemy(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_config: ResMut<EnemySpawnConfig>,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    spawn_config.tick(time.delta());

    if !spawn_config.finished() {
        return;
    }

    if let Ok((player_position)) = player_query.get_single() {
        let angle = (rng.next_u32() % 360) as f32 * std::f32::consts::PI / 180.0;

        let rotation = Quat::from_rotation_z(angle);

        let position = rotation * Vec3::new(0., 200., 0.) + player_position.translation;

        let entity = commands
            .spawn(EnemyBundle {
                collider: Collider::circle(32.),
                aesprite: AsepriteBundle {
                    aseprite: asset_server.load(sprites::TableAnim::PATH),
                    animation: AsepriteAnimation::from(sprites::TableAnim::tags::IDLE),
                    transform: Transform {
                        scale: Vec3::splat(2.),
                        translation: position,
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

        spawn_config.reset();
    }
}

pub fn enemy_follow_player(
    mut player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(&Enemy, &Transform, &mut Velocity)>,
) {
    if let Ok(player_transform) = player_query.get_single_mut() {
        for (enemy, enemy_transform, mut enemy_velocity) in enemy_query.iter_mut() {
            let direction =
                (player_transform.translation - enemy_transform.translation).normalize();
            // *enemy_velocity = Velocity::from_vec(direction * enemy.speed());
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
