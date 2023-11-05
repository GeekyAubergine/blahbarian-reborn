use app::AppPlugin;
use bevy::{prelude::*, window::close_on_esc};
use bevy_aseprite::AsepritePlugin;
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::EntropyPlugin;
use game::GamePlugin;

mod app;
mod game;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Blahbarian".to_owned(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins((
            EntropyPlugin::<ChaCha8Rng>::default(),
            AsepritePlugin,
            AppPlugin,
            GamePlugin,
        ))
        .add_systems(Update, close_on_esc)
        .run();
}
