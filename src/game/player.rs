use bevy::{prelude::*, transform::commands, window::PrimaryWindow};
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle};

use super::{
    animated::{Animated, AnimatedBundle, AnimatedDirection},
    calculate_player_direction_from_mouse,
    camera::{GameCameraGoal, CAMERA_OFFSET_FROM_PLAYER},
    health::{
        spawn_health_bar, {Health, HealthBar},
    },
    physics::{Collider, Velocity},
    weapon::PlayerWeapon,
    EnitityAllegence, GameSet,
};

mod sprites {
    use bevy_aseprite::aseprite;
    aseprite!(pub PlayerAnim, "shark.aseprite");
}

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

pub fn player_input(
    commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    keyboard_input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut player_query: Query<(&mut Player, &Transform, &mut Velocity), Without<GameCameraGoal>>,
) {
    if let Ok((mut player, transform, mut velocity)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        direction = direction.normalize_or_zero();
        *velocity = Velocity::from_vec(direction * player.speed());

        if let Some(cursor_position) = window.single().cursor_position() {
            let cursor_offset_from_center =
                calculate_player_direction_from_mouse(&cursor_position, window.single())
                    .clamp_length_max(CAMERA_OFFSET_FROM_PLAYER);

            if buttons.pressed(MouseButton::Left) && player.weapon_one().can_attack() {
                player.weapon_one_mut().attack(
                    commands,
                    transform.translation,
                    cursor_offset_from_center.normalize_or_zero(),
                );
            }
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (player_input).in_set(GameSet::PlayerInput));
    }
}
