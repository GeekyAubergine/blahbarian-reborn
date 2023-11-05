use bevy::prelude::*;

use super::{animated::AnimatedDirection, GameSet};
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

    pub fn as_animation(&self) -> AnimatedDirection {
        if self.0.length_squared() < 0.1 {
            return AnimatedDirection::Idle;
        }

        if self.0.x <= 0. {
            if self.0.y <= 0. {
                AnimatedDirection::LeftDown
            } else {
                AnimatedDirection::LeftUp
            }
        } else {
            if self.0.y <= 0. {
                AnimatedDirection::RightDown
            } else {
                AnimatedDirection::RightUp
            }
        }
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

#[derive(Bundle)]
struct ColliderBundle {
    pub transform: Transform,
    pub collider: Collider,
}

fn render_debug(mut gizmos: Gizmos, collider_query: Query<(&Collider, &Transform)>) {
    for (collider, transform) in collider_query.iter() {
        match collider {
            Collider::Circle { radius } => {
                gizmos.circle_2d(transform.translation.truncate(), *radius, Color::RED);
            }
        }
    }
}

pub fn update_positions(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.as_vec() * time.delta_seconds();
    }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_positions).in_set(GameSet::Physics));
        #[cfg(debug_assertions)]
        app.add_systems(Update, (render_debug,).in_set(GameSet::Ui));
    }
}
