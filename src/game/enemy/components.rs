use std::time::Duration;

use bevy::prelude::*;

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
