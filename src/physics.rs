use bevy::prelude::*;

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

#[derive(Bundle)]
struct ColliderBundle {
    transform: Transform,
    collider: Collider,
}

impl ColliderBundle {
    pub fn circle(radius: f32, transform: Transform) -> Self {
        Self {
            transform,
            collider: Collider::circle(radius),
        }
    }
}
