use std::time::Duration;

use bevy::{
    prelude::*,
    window::{close_on_esc, PrimaryWindow},
};
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle, AsepritePlugin};
use bevy_ecs_tilemap::{
    prelude::{
        get_tilemap_center_transform, TilemapGridSize, TilemapId, TilemapSize, TilemapTexture,
        TilemapTileSize, TilemapType,
    },
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle, TilemapPlugin,
};

const CAMERA_OFFSET_FROM_PLAYER: f32 = 64.0;

mod sprites {
    use bevy_aseprite::aseprite;

    aseprite!(pub PlayerAnim, "shark.aseprite");
    aseprite!(pub TableAnim, "table.aseprite");
    aseprite!(pub NailAnim, "nail.aseprite");
}

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct CameraGoal;

#[derive(Component)]
struct Health {
    health: i32,
}

#[derive(Component)]
struct Collider {
    radius: f32,
}

#[derive(Component)]
enum Enemy {
    Table { last_melee: f32 },
}

impl Enemy {
    fn speed(&self) -> f32 {
        match self {
            Table => 25.0,
        }
    }

    fn melee_range(&self) -> f32 {
        match self {
            Table => 64.0,
        }
    }

    fn melee_damage(&self) -> i32 {
        match self {
            Table => 10,
        }
    }

    fn melee_cooldown(&self) -> Duration {
        match self {
            Table => Duration::from_secs_f32(0.1),
        }
    }

    fn last_melee(&self) -> f32 {
        match self {
            Enemy::Table { last_melee } => *last_melee,
        }
    }

    fn can_melee(&self, time: f32) -> bool {
        time - self.last_melee() > self.melee_cooldown().as_secs_f32()
    }

    fn set_last_melee(&mut self, time: f32) {
        match self {
            Enemy::Table { last_melee } => *last_melee = time,
        }
    }
}

#[derive(Component)]
struct Projectile {
    velocity: Vec2,
    damage: u32,
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

    let mut transform = get_tilemap_center_transform(&map_size, &grid_size, &map_type, -1.0);

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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        CameraGoal,
        Transform {
            translation: Vec3::new(0., 0., 0.),
            ..Default::default()
        },
    ));

    commands.spawn((
        AsepriteBundle {
            aseprite: asset_server.load(sprites::PlayerAnim::PATH),
            animation: AsepriteAnimation::from(sprites::PlayerAnim::tags::IDLE_LEFT),
            transform: Transform {
                scale: Vec3::splat(3.),
                translation: Vec3::new(0., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        },
        Player { speed: 200.0 },
        Health { health: 100 },
        Collider { radius: 42.0 },
    ));

    spawn_enemy(commands, asset_server);
}

fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AsepriteBundle {
            aseprite: asset_server.load(sprites::TableAnim::PATH),
            animation: AsepriteAnimation::from(sprites::TableAnim::tags::IDLE),
            transform: Transform {
                scale: Vec3::splat(2.),
                translation: Vec3::new(0., 128., 0.),
                ..Default::default()
            },
            ..Default::default()
        },
        Enemy::Table { last_melee: 0.0 },
        Health { health: 100 },
        Collider { radius: 32.0 },
    ));
}

fn update_player_position(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform), Without<CameraGoal>>,
) {
    for (player, mut transform) in player_query.iter_mut() {
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
        transform.translation += time.delta_seconds() * direction * player.speed;
    }
}

fn enemy_follow_player(
    mut player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(&Enemy, &mut Transform)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single_mut() {
        for (enemy, mut enemy_transform) in enemy_query.iter_mut() {
            let direction =
                (player_transform.translation - enemy_transform.translation).normalize();
            enemy_transform.translation += direction * enemy.speed() * time.delta_seconds();
        }
    }
}

fn enemy_melee_player(
    mut player_query: Query<(&mut Health, &Transform, &Collider), With<Player>>,
    mut enemy_query: Query<(&mut Enemy, &Transform, &Collider)>,
    time: Res<Time>,
) {
    if let Ok((mut player_health, player_transform, player_collider)) =
        player_query.get_single_mut()
    {
        for (mut enemy, enemy_transform, enemy_collider) in enemy_query.iter_mut() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            if distance < player_collider.radius + enemy_collider.radius
                && enemy.can_melee(time.elapsed_seconds())
            {
                player_health.health -= enemy.melee_damage();
                enemy.set_last_melee(time.elapsed_seconds());
                println!("Player health: {}", player_health.health);
            }
        }
    }
}

fn update_camera_goal_position(
    player_query: Query<&Transform, (With<Player>, Without<CameraGoal>)>,
    mut camera_goal_query: Query<&mut Transform, (With<CameraGoal>, Without<Player>)>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_goal_transform) = camera_goal_query.get_single_mut() {
            if let Some(position) = window.single().cursor_position() {
                let width = window.single().width();
                let height = window.single().height();

                let cursor_offset_from_center = Vec3::new(
                    position.x - width / 2.0,
                    -(position.y - height / 2.0),
                    0.0,
                )
                .clamp_length_max(CAMERA_OFFSET_FROM_PLAYER);

                camera_goal_transform.translation =
                    player_transform.translation + cursor_offset_from_center;
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

// fn spritemap_fix(mut ev_asset: EventReader<AssetEvent<Image>>, mut assets: ResMut<Assets<Image>>) {
//     for ev in ev_asset.iter() {
//         if let AssetEvent::Created { handle } = ev {
//             if let Some(texture) = assets.get_mut(handle) {
//                 texture.sampler_descriptor = ImageSampler::nearest()
//             }
//         }
//     }
// }

fn render_debug(
    mut gizmos: Gizmos,
    collider_query: Query<(&Collider, &Transform)>,
    camera_goal_position_query: Query<&Transform, With<CameraGoal>>,
) {
    for (collider, tranform) in collider_query.iter() {
        gizmos.circle_2d(tranform.translation.truncate(), collider.radius, Color::RED);
    }

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
        .add_plugins(AsepritePlugin)
        .add_plugins(TilemapPlugin)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_systems(Startup, (setup, setup_tiles))
        .add_systems(
            Update,
            (
                update_player_position,
                enemy_melee_player,
                enemy_follow_player,
                update_camera_goal_position,
                camera_move_to_goal_position,
                #[cfg(debug_assertions)]
                close_on_esc,
                #[cfg(debug_assertions)]
                render_debug,
            ),
        )
        .run();
}
