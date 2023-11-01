use bevy::prelude::*;

use super::components::Velocity;

pub fn update_positions(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.as_vec() * time.delta_seconds();
    }
}
