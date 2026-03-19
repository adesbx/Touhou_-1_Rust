use bevy::app::App;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const PLAYER_DAMAGE: f32 = 10.0;
const PLAYER_SPEED: f32 = 200.0;
const PLAYER_SIZE: f32 = 32.0; // taille du sprite du joueur
const PROJECTILE_SPEED: f32 = 400.0;
const PROJECTILE_SIZE: f32 = 16.0; // taille du sprite projectile
const ANGEL_HP: f32 = 500.0; // taille du sprite ange
const ANGEL_SIZE: f32 = 18.0; // taille du sprite ange
const ZOOM_FACTOR: f32 = 0.5; // zoom caméra

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_ennemy)
        .add_systems(Update, move_player)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, shoot_projectile)
        .add_systems(Update, move_projectile)
        .add_systems(Update, confine_projectile_movement)
        .add_systems(Update, check_collison_enemies)
        .add_systems(Update, check_health)
        .run();
}

fn setup(mut commands: Commands, asset_serv: Res<AssetServer>) {
    commands.spawn((
        Camera2d, 
        Projection::Orthographic(OrthographicProjection { 
            scale: ZOOM_FACTOR,
            ..OrthographicProjection::default_2d()
        }),
    )); 

    let texture = asset_serv.load("characters/character.png");

    commands.spawn((
        Sprite::from_image(texture),
        Transform::from_xyz(0., 0., 0.),
        Player,
    ));
}

fn move_player(
    time:  Res<Time>,
    mut player_transform: Single<&mut Transform, With<Player>>, 
    keyboard: Res<ButtonInput<KeyCode>>
) {
    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    //no more speed bonus in diagonal
    if direction != Vec2::ZERO {
        direction = direction.normalize_or_zero();
    }

    player_transform.translation.x += direction.x * time.delta_secs() * PLAYER_SPEED;
    player_transform.translation.y += direction.y * time.delta_secs() * PLAYER_SPEED;
}

fn confine_player_movement(
    mut player_transform: Single<&mut Transform, With<Player>>, 
    window: Single<&Window, With<PrimaryWindow>>
) {

    let half_player_size: f32 = (PLAYER_SIZE / 2.0) * ZOOM_FACTOR;
    let half_width: f32 = (window.width() / 2.0) * ZOOM_FACTOR;
    let half_height: f32 = (window.height() / 2.0) * ZOOM_FACTOR;

    let x_min = -half_width + half_player_size;
    let x_max = half_width - half_player_size;
    let y_min = -half_height + half_player_size;
    let y_max = half_height - half_player_size;

    let mut translation: Vec3 = player_transform.translation;

    if translation.x < x_min {
        translation.x = x_min
    } else if translation.x > x_max {
        translation.x = x_max
    }

    if translation.y < y_min {
        translation.y = y_min
    } else if translation.y > y_max {
        translation.y = y_max
    }

    player_transform.translation = translation;
}

fn shoot_projectile(
    mut commands: Commands,
    asset_serv: Res<AssetServer>,
    player_transform: Single<&mut Transform, With<Player>>, 
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyK) {
        let texture = asset_serv.load("projectiles/projectile.png");
        commands.spawn((
            Sprite::from_image(texture),
            Transform::from_xyz(
                player_transform.translation.x,
                player_transform.translation.y+10.0,
                player_transform.translation.z
            ),
            Projectile,
        ));
    }
}

fn move_projectile(
    time:  Res<Time>,
    mut projectile_query: Query<&mut Transform, With<Projectile>>, 
) {
    for mut transform in &mut projectile_query {
        let mut direction: Vec2 = Vec2::ZERO;
        direction.y += 1.;
        transform.translation.y += direction.y * time.delta_secs() * PROJECTILE_SPEED;
    }
}

fn confine_projectile_movement(    
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    window: Single<&Window, With<PrimaryWindow>>
) {
    let half_projectile_size: f32 = (PROJECTILE_SIZE / 2.0) * ZOOM_FACTOR;
    let half_height: f32 = (window.height() / 2.0) * ZOOM_FACTOR;

    let y_max = half_height - half_projectile_size;

    for (entity, transform) in &projectile_query {
        if transform.translation.y > y_max {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_ennemy(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    asset_serv: Res<AssetServer>
) {
    let texture = asset_serv.load("enemies/angel.png");
    let half_width = (window.width() / 2.0) * ZOOM_FACTOR;
    let half_height = (window.height() / 2.0) * ZOOM_FACTOR;

    commands.spawn((
        Sprite::from_image(texture),
        Transform::from_xyz(
            half_width - ANGEL_SIZE,
            half_height - ANGEL_SIZE,
            2.0
        ),
        Enemy,
        Health{hp:ANGEL_HP},
    ));    
}

fn check_collison_enemies(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    mut enemy_query: Query<(&Transform, &mut Health), With<Enemy>>,
) {

    for (projectile_entity, projectile_transform) in &projectile_query {
        for (enemy_transform, mut enemy_health) in &mut enemy_query {
            let p1 = projectile_transform.translation.truncate(); // Vec3 -> Vec2
            let p2 = enemy_transform.translation.truncate();
            let distance = p1.distance(p2);
            if distance < (PROJECTILE_SIZE + ANGEL_SIZE) / 2.0 {                
                commands.entity(projectile_entity).despawn();
                enemy_health.hp -= PLAYER_DAMAGE;
                break;
            }
        }
    }
}

fn check_health(
    mut commands: Commands,
    health_query: Query<(Entity, &Health), With<Health>>,
) {

    for (entity, health) in &health_query {
        if health.hp <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
 
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Projectile;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health {
    hp: f32
}
