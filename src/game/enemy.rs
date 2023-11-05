use std::time::Duration;

use bevy::prelude::*;
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle};
use bevy_prng::ChaCha8Rng;
use bevy_rand::resource::GlobalEntropy;
use rand_core::RngCore;

use super::{
    health::{
        {Health, HealthBar},
        spawn_health_bar,
    },
    physics::{Collider, Velocity},
    player::Player,
    EnitityAllegence, EntityTookDamage, GameSet,
};

mod sprites {
    use bevy_aseprite::aseprite;

    aseprite!(pub TableAnim, "table.aseprite");
}

#[derive(Resource)]
pub struct EnemySpawnConfig {
    spawn_interval: f32,
    spawn_difficulty: f32, // % per spawn to increase spawn rate
    spawn_timer: Timer,
}

impl EnemySpawnConfig {
    pub fn new(spawn_interval: f32, spawn_difficulty: f32) -> Self {
        Self {
            spawn_interval,
            spawn_difficulty,
            spawn_timer: Timer::from_seconds(spawn_interval, TimerMode::Repeating),
        }
    }

    pub fn tick(&mut self, delta: Duration) {
        self.spawn_timer.tick(delta);
    }

    pub fn reset(&mut self) {
        self.spawn_timer.reset();
        self.spawn_interval *= 1.0 - self.spawn_difficulty;
        dbg!(self.spawn_interval);
    }

    pub fn finished(&self) -> bool {
        self.spawn_timer.finished()
    }
}

impl Default for EnemySpawnConfig {
    fn default() -> Self {
        Self::new(5.0, 0.1)
    }
}

#[derive(Component)]
pub enum Enemy {
    Table {
        last_melee: f32,
        health_entity: Entity,
    },
}

impl Enemy {
    pub fn speed(&self) -> f32 {
        match self {
            Table => 25.0,
        }
    }

    pub fn melee_range(&self) -> f32 {
        match self {
            Table => 64.0,
        }
    }

    pub fn melee_damage(&self) -> i32 {
        match self {
            Table => 10,
        }
    }

    pub fn melee_cooldown(&self) -> Duration {
        match self {
            Table => Duration::from_secs_f32(0.1),
        }
    }

    pub fn last_melee(&self) -> f32 {
        match self {
            Enemy::Table { last_melee, .. } => *last_melee,
        }
    }

    pub fn can_melee(&self, time: f32) -> bool {
        time - self.last_melee() > self.melee_cooldown().as_secs_f32()
    }

    pub fn set_last_melee(&mut self, time: f32) {
        match self {
            Enemy::Table { last_melee, .. } => *last_melee = time,
        }
    }
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

fn setup_enemy_plugin(mut commands: Commands) {
    commands.insert_resource(EnemySpawnConfig::default());
}

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

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_enemy_plugin));
        app.add_systems(
            Update,
            (
                enemy_follow_player,
                spawn_enemy.run_if(resource_exists::<EnemySpawnConfig>()),
            )
                .in_set(GameSet::Ai),
        );
        app.add_systems(Update, (enemy_melee_player).in_set(GameSet::DealDamage));
    }
}
