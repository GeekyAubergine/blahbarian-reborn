use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(Vec3);

impl Velocity {
    pub fn from_vec(vec: Vec3) -> Self {
        Self(vec)
    }

    pub fn zero() -> Self {
        Self(Vec3::ZERO)
    }

    pub fn as_vec(&self) -> Vec3 {
        self.0
    }
}

#[derive(Component)]
pub enum Collider {
    Circle { radius: f32 },
}

impl Collider {
    pub fn circle(radius: f32) -> Self {
        Self::Circle { radius }
    }

    pub fn is_colliding(
        &self,
        transform: &Transform,
        other: &Self,
        other_transform: &Transform,
    ) -> bool {
        match (self, other) {
            (Self::Circle { radius: r1 }, Self::Circle { radius: r2 }) => {
                transform
                    .translation
                    .distance_squared(other_transform.translation)
                    < (r1 + r2).powi(2)
            }
        }
    }
}
