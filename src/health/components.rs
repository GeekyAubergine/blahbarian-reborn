use bevy::prelude::*;

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
