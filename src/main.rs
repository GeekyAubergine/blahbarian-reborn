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
use physics::Collider;
use weapon::PlayerWeapon;

const CAMERA_OFFSET_FROM_PLAYER: f32 = 64.0;

mod physics;
mod weapon;

mod sprites {
    use bevy_aseprite::aseprite;

    aseprite!(pub PlayerAnim, "shark.aseprite");
    aseprite!(pub TableAnim, "table.aseprite");
    aseprite!(pub NailAnim, "nail.aseprite");
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
struct Player {
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
}

#[derive(Component)]
struct CameraGoal;

#[derive(Component)]
struct Health {
    health: i32,
}

#[derive(Component)]
struct Projectile {
    velocity: Vec3,
    damage: u32,
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
        Collider::circle(32.),
        AsepriteBundle {
            aseprite: asset_server.load(sprites::PlayerAnim::PATH),
            animation: AsepriteAnimation::from(sprites::PlayerAnim::tags::IDLE_LEFT),
            transform: Transform {
                scale: Vec3::splat(3.),
                ..Default::default()
            },
            ..Default::default()
        },
        Player::new(),
        Health { health: 100 },
        EnitityAllegence::Player,
    ));

    spawn_enemy(commands, asset_server);
}

fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Collider::circle(32.),
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
        EnitityAllegence::Enemy,
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

fn move_projectiles(mut projectile_query: Query<(&mut Transform, &Projectile)>, time: Res<Time>) {
    for (mut transform, projectile) in projectile_query.iter_mut() {
        transform.translation += time.delta_seconds() * projectile.velocity;
    }
}

fn player_activates_weapon(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &Transform)>,
    window: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
) {
    if let Some(cursor_position) = window.single().cursor_position() {
        let cursor_offset_from_center =
            calculate_player_direction_from_mouse(&cursor_position, &window.single())
                .clamp_length_max(CAMERA_OFFSET_FROM_PLAYER);

        if let Ok((mut player, player_transform)) = player_query.get_single_mut() {
            if buttons.just_pressed(MouseButton::Left) && player.weapon_one.can_attack() {
                player.weapon_one.attack(
                    commands,
                    player_transform.translation,
                    cursor_offset_from_center.normalize_or_zero(),
                );
            }
        }
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
            if enemy_collider.is_colliding(enemy_transform, player_collider, player_transform) {
                player_health.health -= enemy.melee_damage();
                enemy.set_last_melee(time.elapsed_seconds());
                println!("Player health: {}", player_health.health);
            }
        }
    }
}

fn projectile_hurt_entity(
    mut commands: Commands,
    projectile_query: Query<(
        Entity,
        &Projectile,
        &Transform,
        &Collider,
        &EnitityAllegence,
    )>,
    mut health_query: Query<(&mut Health, &Collider, &Transform, &EnitityAllegence)>,
) {
    for (
        projectile_entity,
        projectile,
        projectile_transform,
        projectile_collider,
        protectile_allegence,
    ) in projectile_query.iter()
    {
        for (mut health, entity_collider, entity_tranform, entity_allegence) in
            health_query.iter_mut()
        {
            if entity_allegence == protectile_allegence {
                continue;
            }

            if entity_collider.is_colliding(
                entity_tranform,
                projectile_collider,
                projectile_transform,
            ) {
                health.health -= projectile.damage as i32;
                println!("Entity health: {}", health.health);
                commands.entity(projectile_entity).despawn();
            }
        }
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
    collider_query: Query<(&Collider, &Transform)>,
    camera_goal_position_query: Query<&Transform, With<CameraGoal>>,
) {
    for (collider, transform) in collider_query.iter() {
        match collider {
            Collider::Circle { radius } => {
                gizmos.circle_2d(transform.translation.truncate(), *radius, Color::RED);
            }
        }
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
        .insert_resource(Settings::new())
        .add_systems(Startup, (setup, setup_tiles))
        .add_systems(
            Update,
            (
                update_player_position,
                player_activates_weapon,
                move_projectiles,
                enemy_melee_player,
                enemy_follow_player,
                update_camera_goal_position,
                camera_move_to_goal_position,
                projectile_hurt_entity,
                #[cfg(debug_assertions)]
                close_on_esc,
                #[cfg(debug_assertions)]
                render_debug,
            ),
        )
        .run();
}
