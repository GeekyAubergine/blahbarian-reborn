use std::{f32::consts::{E, PI}, time::Duration};

use bevy::{
    prelude::*, render::texture::ImageSampler, sprite::MaterialMesh2dBundle, window::close_on_esc,
};
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle, AsepritePlugin};

const CAMERA_LAG: f32 = 0.05;

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
            Table => 50.0,
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        AsepriteBundle {
            aseprite: asset_server.load(sprites::PlayerAnim::PATH),
            animation: AsepriteAnimation::from(sprites::PlayerAnim::tags::WALK),
            transform: Transform {
                scale: Vec3::splat(2.),
                translation: Vec3::new(0., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        },
        Player { speed: 100.0 },
        Health { health: 100 },
        Collider { radius: 32.0 },
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
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in query.iter_mut() {
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
        transform.translation += time.delta_seconds() * direction * player.speed;
    }
}

fn enemy_follow_player(
    mut player_query: Query<(&Transform), (With<Player>, Without<Enemy>)>,
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

fn camera_follow_player(
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let delta = player_transform.translation - camera_transform.translation;
            camera_transform.translation += delta * CAMERA_LAG;
        }
    }
}

fn spritemap_fix(mut ev_asset: EventReader<AssetEvent<Image>>, mut assets: ResMut<Assets<Image>>) {
    for ev in ev_asset.iter() {
        if let AssetEvent::Created { handle } = ev {
            if let Some(texture) = assets.get_mut(handle) {
                texture.sampler_descriptor = ImageSampler::nearest()
            }
        }
    }
}

fn render_debug(mut gizmos: Gizmos, collider_query: Query<(&Collider, &Transform)>) {
    for (collider, tranform) in collider_query.iter() {
        gizmos.circle_2d(tranform.translation.truncate(), collider.radius, Color::RED);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AsepritePlugin)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spritemap_fix,
                update_player_position,
                camera_follow_player,
                enemy_melee_player,
                enemy_follow_player,
                #[cfg(debug_assertions)]
                close_on_esc,
                #[cfg(debug_assertions)]
                render_debug,
            ),
        )
        .run();
}
