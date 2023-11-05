use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    damage: i32,
}

impl Projectile {
    pub fn new(damage: i32) -> Self {
        Self { damage }
    }

    pub fn damage(&self) -> i32 {
        self.damage
    }
}
