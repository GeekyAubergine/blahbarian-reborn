use std::time::Duration;

use bevy::prelude::*;
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle};

use self::{
    components::Enemy,
    systems::{enemy_follow_player, enemy_melee_player, spawn_enemy},
};

use super::{
    health::components::Health,
    physics::components::{Collider, Velocity},
    EnitityAllegence, GameSet,
};

pub mod components;
pub mod systems;

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
