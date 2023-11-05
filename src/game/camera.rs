use bevy::{prelude::*, window::PrimaryWindow};

use crate::app::Settings;

use super::{calculate_player_direction_from_mouse, player::Player, GameSet};

pub const CAMERA_OFFSET_FROM_PLAYER: f32 = 64.0;

#[derive(Component, Debug)]
pub struct GameCamera;

#[derive(Debug, PartialEq, Eq)]
pub enum GameCameraFollowMode {
    Sticky,
    Leading,
}

#[derive(Component)]
pub struct GameCameraGoal;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCamera));

    commands.spawn((
        GameCameraGoal,
        Transform {
            translation: Vec3::new(0., 0., 0.),
            ..Default::default()
        },
    ));
}

fn update_camera_goal_position(
    player_query: Query<&Transform, (With<Player>, Without<GameCameraGoal>)>,
    mut camera_goal_query: Query<&mut Transform, (With<GameCameraGoal>, Without<Player>)>,
    window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<Settings>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_goal_transform) = camera_goal_query.get_single_mut() {
            if let Some(position) = window.single().cursor_position() {
                match settings.camera_follow_mode() {
                    GameCameraFollowMode::Sticky => {
                        camera_goal_transform.translation = player_transform.translation;
                    }
                    GameCameraFollowMode::Leading => {
                        let cursor_offset_from_center =
                            calculate_player_direction_from_mouse(&position, window.single())
                                .clamp_length_max(CAMERA_OFFSET_FROM_PLAYER);

                        camera_goal_transform.translation =
                            player_transform.translation + cursor_offset_from_center;
                    }
                }
            } else {
                camera_goal_transform.translation = player_transform.translation;
            }
        }
    }
}

fn camera_move_to_goal_position(
    camera_goal_query: Query<&Transform, (With<GameCameraGoal>, Without<GameCamera>)>,
    mut camera_query: Query<&mut Transform, (With<GameCamera>, Without<GameCameraGoal>)>,
) {
    if let Ok(camera_goal_transform) = camera_goal_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let delta = camera_goal_transform.translation - camera_transform.translation;
            camera_transform.translation += delta * 0.5;
        }
    }
}

fn render_debug(
    mut gizmos: Gizmos,
    camera_goal_position_query: Query<&Transform, With<GameCameraGoal>>,
) {
    if let Ok(camera_goal_transform) = camera_goal_position_query.get_single() {
        gizmos.circle_2d(
            camera_goal_transform.translation.truncate(),
            10.0,
            Color::GREEN,
        );
    }
}

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup,));
        app.add_systems(
            Update,
            (camera_move_to_goal_position, update_camera_goal_position).in_set(GameSet::Ui),
        );
        #[cfg(debug_assertions)]
        app.add_systems(Update, (render_debug,).in_set(GameSet::Ui));
    }
}
