use bevy::prelude::*;

use crate::game::weapon::PlayerWeapon;

#[derive(Component)]
pub struct Player {
    speed: f32,
    weapon_one: PlayerWeapon,
    weapon_two: Option<PlayerWeapon>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            speed: 200.0,
            weapon_one: PlayerWeapon::axe(),
            weapon_two: None,
        }
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn weapon_one(&self) -> &PlayerWeapon {
        &self.weapon_one
    }

    pub fn weapon_one_mut(&mut self) -> &mut PlayerWeapon {
        &mut self.weapon_one
    }

    pub fn weapon_two(&self) -> Option<&PlayerWeapon> {
        self.weapon_two.as_ref()
    }
}
