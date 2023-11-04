use app::AppPlugin;
use bevy::{
    prelude::*,
    window::{close_on_esc, PrimaryWindow},
};
use bevy_aseprite::AsepritePlugin;
use bevy_ecs_tilemap::{
    prelude::{
        get_tilemap_center_transform, TilemapGridSize, TilemapId, TilemapSize, TilemapTexture,
        TilemapTileSize, TilemapType,
    },
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle, TilemapPlugin,
};
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::EntropyPlugin;
use game::GamePlugin;

mod app;
mod game;

mod sprites {
    use bevy_aseprite::aseprite;

    aseprite!(pub PlayerAnim, "shark.aseprite");
    aseprite!(pub TableAnim, "table.aseprite");
    aseprite!(pub NailAnim, "nail.aseprite");
}

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
