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

use self::{
    camera::GameCameraPlugin,
    enemy::EnemyPlugin,
    health::HealthPlugin,
    physics::PhysicsPlugin,
    player::{components::Player, PlayerPlugin},
    projectile::ProjectilePlugin,
};

pub mod camera;
pub mod enemy;
pub mod health;
pub mod physics;
pub mod player;
pub mod projectile;
pub mod ui;
pub mod weapon;

#[derive(SystemSet, Hash, PartialEq, Eq, Clone, Debug)]
enum GameSet {
    PlayerInput,
    Physics,
    Ai,
    DealDamage,
    ResolveDamage,
    Ui,
}

#[derive(Event, Debug)]
pub struct EntityTookDamage {
    entity: Entity,
    damage: i32,
}

impl EntityTookDamage {
    pub fn new(entity: Entity, damage: i32) -> Self {
        Self { entity, damage }
    }
}

#[derive(Component, Debug, PartialEq, Eq)]
pub enum EnitityAllegence {
    Player,
    Enemy,
}

fn setup_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tiles_texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size = TilemapSize { x: 16, y: 16 };

    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(0),
                    ..Default::default()
                })
                .id();

            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };

    let grid_size = TilemapGridSize { x: 32.0, y: 32.0 };

    let map_type = TilemapType::default();

    let mut transform = get_tilemap_center_transform(&map_size, &grid_size, &map_type, -5.0);

    transform.translation *= 2.0;
    transform.scale = Vec3::splat(2.0);

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(tiles_texture_handle),
        tile_size,
        transform,
        ..Default::default()
    });

    // Add atlas to array texture loader so it's preprocessed before we need to use it.
    // Only used when the atlas feature is off and we are using array textures.
    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    {
        array_texture_loader.add(TilemapArrayTexture {
            texture: TilemapTexture::Single(asset_server.load("tiles.png")),
            tile_size,
            ..Default::default()
        });
    }
}

fn calculate_player_direction_from_mouse(cursor_position: &Vec2, window: &Window) -> Vec3 {
    let width = window.width();
    let height = window.height();

    Vec3::new(
        cursor_position.x - width / 2.0,
        -(cursor_position.y - height / 2.0),
        0.0,
    )
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityTookDamage>();
        app.configure_set(Update, GameSet::PlayerInput.before(GameSet::Physics));
        app.configure_set(Update, GameSet::Physics.before(GameSet::DealDamage));
        app.configure_set(Update, GameSet::DealDamage.before(GameSet::ResolveDamage));
        app.configure_set(Update, GameSet::ResolveDamage.before(GameSet::Ui));
        app.add_plugins((
            GameCameraPlugin,
            TilemapPlugin,
            PhysicsPlugin,
            PlayerPlugin,
            EnemyPlugin,
            ProjectilePlugin,
            HealthPlugin,
        ));
        app.add_systems(Startup, (setup_tiles));
    }
}
