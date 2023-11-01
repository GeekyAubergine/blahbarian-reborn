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
use enemy::EnemyPlugin;
use health::{components::Health, HealthPlugin};
use physics::{components::Collider, PhysicsPlugin};
use player::{components::Player, PlayerPlugin};
use projectile::ProjectilePlugin;

const CAMERA_OFFSET_FROM_PLAYER: f32 = 64.0;

mod enemy;
mod health;
mod physics;
mod player;
mod projectile;
mod weapon;

mod sprites {
    use bevy_aseprite::aseprite;

    aseprite!(pub PlayerAnim, "shark.aseprite");
    aseprite!(pub TableAnim, "table.aseprite");
    aseprite!(pub NailAnim, "nail.aseprite");
}

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
struct EntityTookDamage {
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

#[derive(Debug, PartialEq, Eq)]
enum CameraFollowMode {
    Sticky,
    Leading,
}

#[derive(Resource)]
pub struct Settings {
    camera_follow_mode: CameraFollowMode,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            camera_follow_mode: CameraFollowMode::Sticky,
        }
    }
}

#[derive(Component)]
struct CameraGoal;

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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        CameraGoal,
        Transform {
            translation: Vec3::new(0., 0., 0.),
            ..Default::default()
        },
    ));
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

fn update_camera_goal_position(
    player_query: Query<&Transform, (With<Player>, Without<CameraGoal>)>,
    mut camera_goal_query: Query<&mut Transform, (With<CameraGoal>, Without<Player>)>,
    window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<Settings>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_goal_transform) = camera_goal_query.get_single_mut() {
            if let Some(position) = window.single().cursor_position() {
                match settings.camera_follow_mode {
                    CameraFollowMode::Sticky => {
                        camera_goal_transform.translation = player_transform.translation;
                    }
                    CameraFollowMode::Leading => {
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
    camera_goal_query: Query<&Transform, (With<CameraGoal>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<CameraGoal>)>,
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
    camera_goal_position_query: Query<&Transform, With<CameraGoal>>,
) {
    if let Ok(camera_goal_transform) = camera_goal_position_query.get_single() {
        gizmos.circle_2d(
            camera_goal_transform.translation.truncate(),
            10.0,
            Color::GREEN,
        );
    }
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
        .insert_resource(Settings::new())
        .add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
        .add_event::<EntityTookDamage>()
        .configure_set(Update, GameSet::PlayerInput.before(GameSet::Physics))
        .configure_set(Update, GameSet::Physics.before(GameSet::DealDamage))
        .configure_set(Update, GameSet::DealDamage.before(GameSet::ResolveDamage))
        .configure_set(Update, GameSet::ResolveDamage.before(GameSet::Ui))
        .add_plugins((
            AsepritePlugin,
            TilemapPlugin,
            PhysicsPlugin,
            PlayerPlugin,
            EnemyPlugin,
            ProjectilePlugin,
            HealthPlugin,
        ))
        // .add_plugins(HealthBarPlugin::<Health>::default())
        .add_systems(Startup, (setup, setup_tiles))
        .add_systems(Update, (camera_move_to_goal_position,).in_set(GameSet::Ui))
        .add_systems(
            Update,
            (
                update_camera_goal_position,
                #[cfg(debug_assertions)]
                render_debug,
                #[cfg(debug_assertions)]
                close_on_esc,
            )
                .in_set(GameSet::Ui),
        )
        .run();
}
