use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    calculate_player_direction_from_mouse, physics::components::Velocity, CameraGoal,
    CAMERA_OFFSET_FROM_PLAYER,
};

use super::components::Player;

pub fn player_input(
    commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    keyboard_input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut player_query: Query<(&mut Player, &Transform, &mut Velocity), Without<CameraGoal>>,
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

