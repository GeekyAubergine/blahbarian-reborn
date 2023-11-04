use bevy::{prelude::*, window::close_on_esc};

use crate::game::camera::GameCameraFollowMode;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    Paused,
}

#[derive(Resource)]
pub struct Settings {
    camera_follow_mode: GameCameraFollowMode,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            camera_follow_mode: GameCameraFollowMode::Sticky,
        }
    }

    pub fn camera_follow_mode(&self) -> &GameCameraFollowMode {
        &self.camera_follow_mode
    }
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Settings::new());
        #[cfg(debug_assertions)]
        app.add_systems(Update, close_on_esc);
    }
}
