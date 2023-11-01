use std::time::{Duration, Instant};

use bevy::prelude::*;

use crate::{
    physics::components::{Collider, Velocity},
    projectile::{components::Projectile, ProjectileBundle},
    EnitityAllegence,
};

const AXE_VELOCITY: f32 = 512.;

#[derive(Component)]
pub enum PlayerWeapon {
    Axe { last_attack: Option<Instant> },
}

fn spawn_axe(mut commands: Commands, player_transform: Vec3, player_facing: Vec3) {
    let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    transform.rotate(Quat::from_rotation_z(player_facing.angle_between(Vec3::X)));
    transform.translation += player_transform + player_facing * 0.5;

    let collider = Collider::circle(16.);

    commands.spawn(ProjectileBundle {
        transform,
        velocity: Velocity::from_vec(player_facing * AXE_VELOCITY),
        collider,
        projectile: Projectile::new(25),
        allegence: EnitityAllegence::Player,
    });
}

impl PlayerWeapon {
    pub fn axe() -> Self {
        Self::Axe { last_attack: None }
    }

    pub fn attack(&mut self, commands: Commands, player_transform: Vec3, player_facing: Vec3) {
        if !self.can_attack() {
            return;
        }

        match self {
            Self::Axe { .. } => spawn_axe(commands, player_transform, player_facing),
        }

        match self {
            Self::Axe { last_attack } => *last_attack = Some(Instant::now()),
        }
    }

    fn cooldown(&self) -> Duration {
        match self {
            Self::Axe { .. } => Duration::from_millis(500),
        }
    }

    fn cooldown_remaining(&self) -> Duration {
        let last_attack = match self {
            Self::Axe { last_attack } => last_attack,
        };

        match last_attack {
            Some(last_attack) => {
                let time_since_last_attack = Instant::now().duration_since(*last_attack);
                self.cooldown()
                    .checked_sub(time_since_last_attack)
                    .unwrap_or_default()
            }
            None => Duration::from_secs(0),
        }
    }

    pub fn can_attack(&self) -> bool {
        self.cooldown_remaining() <= Duration::from_secs(0)
    }

    fn damage(&self) -> u32 {
        match self {
            Self::Axe { .. } => 1,
        }
    }
}
