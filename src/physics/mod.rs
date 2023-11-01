use bevy::prelude::*;

use crate::GameSet;

use self::{components::Collider, systems::update_positions};

pub mod components;
pub mod systems;

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

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_positions).in_set(GameSet::Physics));
        #[cfg(debug_assertions)]
        app.add_systems(Update, (render_debug,).in_set(GameSet::Ui));
    }
}
