use bevy::{prelude::*, transform::commands};
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle};

use self::{components::Player, systems::player_input};

use super::{
    animated::{Animated, AnimatedBundle, AnimatedDirection},
    health::{
        components::{Health, HealthBar},
        spawn_health_bar,
    },
    physics::components::{Collider, Velocity},
    EnitityAllegence, GameSet,
};

pub mod components;
pub mod systems;

mod sprites {
    use bevy_aseprite::aseprite;

    aseprite!(pub PlayerAnim, "shark.aseprite");
}

#[derive(Bundle)]
pub struct PlayerBundle {
    collider: Collider,
    aesprite: AsepriteBundle,
    player: Player,
    allegence: EnitityAllegence,
    health: Health,
    velocity: Velocity,
    animated: AnimatedBundle,
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let entity = commands
        .spawn(PlayerBundle {
            collider: Collider::circle(32.),
            aesprite: AsepriteBundle {
                aseprite: asset_server.load(sprites::PlayerAnim::PATH),
                animation: AsepriteAnimation::from(sprites::PlayerAnim::tags::IDLE),
                transform: Transform {
                    scale: Vec3::splat(3.),
                    translation: Vec3::new(0., 0., 1.),
                    ..Default::default()
                },
                ..Default::default()
            },
            player: Player::new(),
            allegence: EnitityAllegence::Player,
            health: Health::new(100),
            velocity: Velocity::zero(),
            animated: AnimatedBundle {
                animated: Animated::new(
                    Some(sprites::PlayerAnim::tags::IDLE.to_string()),
                    sprites::PlayerAnim::tags::RUN_DOWN_LEFT.to_string(),
                    sprites::PlayerAnim::tags::RUN_DOWN_RIGHT.to_string(),
                    sprites::PlayerAnim::tags::RUN_UP_LEFT.to_string(),
                    sprites::PlayerAnim::tags::RUN_UP_RIGHT.to_string(),
                ),
                animated_direction: AnimatedDirection::default(),
            },
        })
        .id();

    spawn_health_bar(commands, meshes, materials, HealthBar::new(entity, 24.));
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (player_input).in_set(GameSet::PlayerInput));
    }
}
