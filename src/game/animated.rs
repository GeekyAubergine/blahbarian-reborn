use bevy::prelude::*;
use bevy_aseprite::anim::{AsepriteAnimation, self};

use super::{physics::components::Velocity, GameSet};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimatedDirection {
    Idle,
    LeftDown,
    LeftUp,
    RightDown,
    RightUp,
}

impl Default for AnimatedDirection {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Component)]
pub struct Animated {
    idle: Option<String>,
    down_left: String,
    down_right: String,
    up_left: String,
    up_right: String,
}

impl Animated {
    pub fn new(
        idle: Option<String>,
        down_left: String,
        down_right: String,
        up_left: String,
        up_right: String,
    ) -> Self {
        Self {
            idle,
            down_left,
            down_right,
            up_left,
            up_right,
        }
    }
}

#[derive(Bundle)]
pub struct AnimatedBundle {
    pub animated: Animated,
    pub animated_direction: AnimatedDirection,
}

fn update_animated_direction(mut query: Query<(&mut AnimatedDirection, &Velocity)>) {
    for (mut animated_direction, velocity) in query.iter_mut() {
        let next_direction = velocity.as_animation();
        
        if *animated_direction != next_direction {
            dbg!(&next_direction);
            *animated_direction = next_direction;
        }
    }
}

fn update_animation(
    mut query: Query<
        (&mut AsepriteAnimation, &Animated, &AnimatedDirection),
        Changed<AnimatedDirection>,
    >,
) {
    for (mut animation, animated, direction) in query.iter_mut() {
        let animation_tag = match direction {
            AnimatedDirection::Idle => {
                dbg!(&animated.idle);
                if let Some(idle_animation) = &animated.idle {
                    idle_animation.clone()
                } else {
                    animated.down_left.clone()
                }
            }
            AnimatedDirection::LeftDown => animated.down_left.clone(),
            AnimatedDirection::LeftUp => animated.up_left.clone(),
            AnimatedDirection::RightDown => animated.down_right.clone(),
            AnimatedDirection::RightUp => animated.up_right.clone(),
        };

        dbg!(&animation_tag);

        *animation = AsepriteAnimation::from(animation_tag);
    }
}

pub struct AnimatedPlugin;

impl Plugin for AnimatedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_animated_direction,
                update_animation.after(update_animated_direction),
            )
                .in_set(GameSet::Animation),
        );
    }
}
