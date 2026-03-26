use bevy::app::App;
use bevy::prelude::*;
use rand::Rng;
use serde::Deserialize;
use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use futures_lite::AsyncReadExt;

const INVINCIBILITY_TIME: f32 = 2.0;
const PLAYER_DAMAGE: f32 = 10.0;
const PLAYER_HP: f32 = 3.0;
const PLAYER_SPEED: f32 = 200.0;
const PLAYER_SIZE: f32 = 32.0; // taille du sprite du joueur
const PROJECTILE_SPEED: f32 = 400.0;
const PROJECTILE_SIZE: f32 = 16.0; // taille du sprite projectile
const ANGEL_HP: f32 = 100.0; 
const ANGEL_SIZE: f32 = 18.0; // taille du sprite ange
const CHERUB_SIZE: f32 = 18.0;
const CROSS_PROJECTILE_SPEED: f32 = 200.0;
const CROSS_PROJECTILE_SIZE: f32 = 16.0; // taille du sprite projectile
const POWER_UP_SIZE: f32 = 14.0;

const GAME_WIDTH: f32 = 384.0;
const GAME_HEIGHT: f32 = 448.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_asset::<LevelData>()
        .register_asset_loader(LevelDataLoader)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, shoot_projectile)
        .add_systems(Update, move_projectile)
        .add_systems(Update, confine_projectile_movement)
        .add_systems(Update, check_collison_enemies)
        .add_systems(Update, check_health)
        .add_systems(Update, move_enemies)
        .add_systems(Update, enemies_shoot_projectiles)
        .add_systems(Update, move_enemy_projectiles)
        .add_systems(Update, check_collison_projectile_player)
        .add_systems(Update, update_health_ui)
        .add_systems(Update, update_damage_ui)
        .add_systems(Update, move_power_up)
        .add_systems(Update, check_collison_power_up)
        .add_systems(Update, spawn_from_level_data)
        .run();
}

fn setup(mut commands: Commands, asset_serv: Res<AssetServer>) {   
    let handle = asset_serv.load("enemies.ron");
    commands.insert_resource(LevelHandle(handle));

    commands.spawn((
        Camera2d, 
        Projection::Orthographic(OrthographicProjection { 
            scaling_mode: bevy::camera::ScalingMode::AutoMin { 
                min_width: GAME_WIDTH, 
                min_height: GAME_HEIGHT
            },
            ..OrthographicProjection::default_2d()
        }),
    )); 

    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.1, 0.1, 0.9),
            Vec2::new(50.0, GAME_HEIGHT * 2.0),
        ),
        Transform::from_xyz(-GAME_WIDTH / 2.0 - 50.0 / 2.0, 0.0, -10.0),
    ));

    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.1, 0.1, 0.9),
            Vec2::new(200.0, GAME_HEIGHT * 2.0),
        ),
        Transform::from_xyz(GAME_WIDTH / 2.0 + 200.0 / 2.0, 0.0, -10.0),
    ));

    let texture = asset_serv.load("characters/character.png");

    commands.spawn((
        Sprite::from_image(texture),
        Transform::from_xyz(0., 0., 0.),
        Player { last_hit: 0.0},
        Health { hp: PLAYER_HP},
        Damage { damage: PLAYER_DAMAGE}
    ));

    commands.spawn((
        Text::new("HP: 3"), 
        TextFont {
            font_size: 40.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(GAME_HEIGHT/2.0),
            left: Val::Px(GAME_HEIGHT/2.0 + 800.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.0, 0.0)), 
        PlayerHealthText, 
    ));

    commands.spawn((
        Text::new("Damage: 10"), 
        TextFont {
            font_size: 40.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(GAME_HEIGHT/2.0 + 30.0),
            left: Val::Px(GAME_HEIGHT/2.0 + 800.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.0, 0.0)), 
        PlayerDamageText, 
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
) {

    let half_player_size: f32 = PLAYER_SIZE / 2.0;
    let half_width: f32 = GAME_WIDTH / 2.0;
    let half_height: f32 = GAME_HEIGHT / 2.0;

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
    mut player_query: Single<(&Transform, &mut Damage), With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.pressed(KeyCode::KeyK) {
        let texture = asset_serv.load("projectiles/projectile.png");
        let (transform, damage_player) = &mut *player_query; // possiblement sale voir pour faire autrement

        let base_x = transform.translation.x;
        let base_y = transform.translation.y + 10.0;
        let z = transform.translation.z;

        if damage_player.damage < 50.0 {
            commands.spawn((
                Sprite::from_image(texture),
                Transform::from_xyz(
                    base_x,
                    base_y,
                    z
                ),
                Projectile { direction: Vec2::new(0.0, 1.0), speed: PROJECTILE_SPEED},
            ));
        } else if damage_player.damage < 100.0 {
            commands.spawn((
                Sprite::from_image(texture.clone()),
                Transform::from_xyz(base_x, base_y, z),
                Projectile { direction: Vec2::new(0.0, 1.0), speed: PROJECTILE_SPEED},
            ));

            commands.spawn((
                Sprite::from_image(texture.clone()),
                Transform::from_xyz(base_x, base_y, z),
                Projectile { direction: Vec2::new(-0.5, 1.0).normalize(), speed: PROJECTILE_SPEED},
            ));

            commands.spawn((
                Sprite::from_image(texture),
                Transform::from_xyz(base_x, base_y, z),
                Projectile { direction: Vec2::new(0.5, 1.0).normalize(), speed: PROJECTILE_SPEED},
            ));
        }

    }
}

fn move_projectile(
    time:  Res<Time>,
    mut projectile_query: Query<(&mut Transform, &Projectile)>,
) {
    for (mut transform, projectile) in &mut projectile_query {
        let movement = projectile.direction.normalize() * projectile.speed * time.delta_secs();
        transform.translation += movement.extend(0.0);
    }
}

fn confine_projectile_movement(    
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    enemy_projectile_query: Query<(Entity, &Transform), With<EnemyProjectile>>,

) {
    let half_projectile_size: f32 = PROJECTILE_SIZE / 2.0;
    let half_height: f32 = GAME_HEIGHT / 2.0;
    let half_width: f32 = GAME_WIDTH / 2.0;


    let x_min = -half_width + half_projectile_size;
    let x_max = half_width - half_projectile_size;
    let y_min = -half_height + half_projectile_size;
    let y_max = half_height - half_projectile_size;

    for (entity, transform) in &projectile_query {
        if transform.translation.y > y_max || transform.translation.y < y_min || transform.translation.x > x_max || transform.translation.x < x_min {
            commands.entity(entity).despawn();
        }
    }

    for (entity, transform) in &enemy_projectile_query {
        if transform.translation.y > y_max || transform.translation.y < y_min || transform.translation.x > x_max || transform.translation.x < x_min {
            commands.entity(entity).despawn();
        }
    }
}

// fn spawn_enemies(
//     time: Res<Time>,
//     mut commands: Commands,
//     asset_serv: Res<AssetServer>
// ) {
//     let texture = asset_serv.load("enemies/angel.png");
//     let spawn_t: f32 = time.elapsed_secs();

//     let half_width = GAME_WIDTH / 2.0;
//     let half_height = GAME_HEIGHT / 2.0;
//     let top_y = half_height;

//     // --- GROUPE GAUCHE (Vont vers la DROITE : direction = 1.0) ---
//     let left_x = -half_width + ANGEL_SIZE;
//     for i in 0..3 {
//         commands.spawn((
//             Sprite::from_image(texture.clone()),
//             Transform::from_xyz(left_x, top_y - (i as f32 * ANGEL_SIZE * 1.2), 2.0),
//             Enemy,
//             Health { hp: ANGEL_HP },
//             EnemyMovement { spawn_time: spawn_t, direction: 1.0 },
//         ));
//     }

//     // --- GROUPE DROITE (Vont vers la GAUCHE : direction = -1.0) ---
//     let right_x = half_width - ANGEL_SIZE;
//     for i in 0..3 {
//         commands.spawn((
//             Sprite::from_image(texture.clone()),
//             Transform::from_xyz(right_x, top_y - (i as f32 * ANGEL_SIZE * 1.2), 2.0),
//             Enemy,
//             Health { hp: ANGEL_HP },
//             EnemyMovement { spawn_time: spawn_t, direction: -1.0 },
//         ));
//     }

//     // --- GROUPE MILIEU (Descente "droite" : direction = 0.0) ---
//     let x_offset = ANGEL_SIZE; 
//     for x_side in [-1.0, 1.0] { 
//         commands.spawn((
//             Sprite::from_image(texture.clone()),
//             Transform::from_xyz(x_side * x_offset, top_y, 2.0),
//             Enemy,
//             Health { hp: ANGEL_HP },
//             EnemyMovement { spawn_time: spawn_t, direction: 0.0 },
//         ));
//     }

// }

fn spawn_from_level_data(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_assets: Res<Assets<LevelData>>,
    level_handle: Res<LevelHandle>, 
    mut next_index: Local<usize>,
) {    
    if let Some(level) = level_assets.get(&level_handle.0) {
        let current_time = time.elapsed_secs();

        while *next_index < level.waves.len() && current_time >= level.waves[*next_index].spawn_time {
            let wave = &level.waves[*next_index];
            
            let mut text = "";
            if wave.variety == 'a'{
                text = "enemies/angel.png";
            }
            else if wave.variety == 'c'{
                text = "enemies/cherubien.png";
            }
            
            commands.spawn((
                Sprite::from_image(asset_server.load(text)),
                Transform::from_translation(wave.pos.extend(2.0)),
                Enemy { variety: wave.variety},
                Health { hp: wave.hp.hp },
                EnemyMovement { spawn_time: current_time, direction: wave.direction },
            ));

            *next_index += 1;
        }
    }
}

fn move_enemies(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &EnemyMovement), With<Enemy>>,
) {
    let elapsed = time.elapsed_secs();

    for (mut transform, movement) in &mut query {
        transform.translation.y -= 50.0 * time.delta_secs();

        let local_time = elapsed - movement.spawn_time;
        
        let offset_x = (local_time * 1.5).sin() * 75.0 * movement.direction;

        transform.translation.x += offset_x * time.delta_secs();
    }
}

fn check_collison_enemies(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    mut enemy_query: Query<(&Transform, &mut Health), With<Enemy>>,
    player_query: Single<&Damage, With<Player>>,
) {

    for (projectile_entity, projectile_transform) in &projectile_query {
        for (enemy_transform, mut enemy_health) in &mut enemy_query {
            let p1 = projectile_transform.translation.truncate(); // Vec3 -> Vec2
            let p2 = enemy_transform.translation.truncate();
            let distance = p1.distance(p2);
            if distance < (PROJECTILE_SIZE + ANGEL_SIZE) / 2.0 {                
                commands.entity(projectile_entity).despawn();
                enemy_health.hp -= player_query.damage;
                break;
            }
        }
    }
}

fn check_collison_projectile_player(
    time: Res<Time>,
    mut commands: Commands,
    enemy_projectile_query: Query<(Entity, &Transform), With<EnemyProjectile>>,
    mut player_query: Single<(&Transform, &mut Health, &mut Player), With<Player>>,
) {
    let (transform, health, player) = &mut *player_query; // possiblement sale voir pour faire autrement
    if time.elapsed_secs() - player.last_hit > INVINCIBILITY_TIME  {
        for (projectile_entity, projectile_transform) in &enemy_projectile_query {
            let p1 = projectile_transform.translation.truncate(); // Vec3 -> Vec2
            let p2 = transform.translation.truncate();
            let distance = p1.distance(p2);
            if distance < (PROJECTILE_SIZE + PLAYER_SIZE) / 2.0 {                
                commands.entity(projectile_entity).despawn();
                health.hp -= 1.0;
                player.last_hit = time.elapsed_secs();
                break;
            }
            
        }
    }
}

fn enemies_shoot_projectiles(
    mut commands: Commands,
    asset_serv: Res<AssetServer>,
    enemy_transform: Query<(&Transform, &Enemy), With<Enemy>>,
    player_transform: Single<&Transform, With<Player>>,
) {
    let player_pos = player_transform.translation.truncate(); 

    for (transform, enemy) in &enemy_transform {
        let mut rng = rand::thread_rng();

        if rng.gen_range(1..101) == 1 && enemy.variety == 'a' {  
            let enemy_pos = transform.translation.truncate();
            let direction = (player_pos - enemy_pos).normalize_or_zero();

            let texture: Handle<Image> = asset_serv.load("projectiles/projectile_cross.png");
            commands.spawn((
                Sprite::from_image(texture),
                Transform::from_translation(transform.translation),
                EnemyProjectile{velocity: direction*CROSS_PROJECTILE_SPEED},
            ));
          }
    }
}

fn move_enemy_projectiles(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &EnemyProjectile)>,
) {
    for (mut transform, projectile) in &mut query {
        let movement = projectile.velocity.extend(0.0) * time.delta_secs();
        transform.translation += movement;
    }
}

fn move_power_up(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<PowerUp>>,
) {
    for mut transform  in &mut query {
        transform.translation.y -= 40.0 * time.delta_secs();

    }
}

fn check_collison_power_up(
    mut commands: Commands,
    power_up_query: Query<(Entity, &Transform), With<PowerUp>>,
    mut player_query: Single<(&Transform, &mut Damage), With<Player>>,
) {
    let (transform, damage_player) = &mut *player_query; // possiblement sale voir pour faire autrement

    for (power_entity, power_transform) in &power_up_query {
            let p1 = power_transform.translation.truncate(); // Vec3 -> Vec2
            let p2 = transform.translation.truncate();
            let distance = p1.distance(p2);
            if distance < (POWER_UP_SIZE + PLAYER_SIZE) / 2.0 {                
                damage_player.damage += PLAYER_DAMAGE;
                commands.entity(power_entity).despawn();
                break;
            }
    }
}

fn check_health(
    mut commands: Commands,
    asset_serv: Res<AssetServer>,
    health_query: Query<(Entity, &Transform ,&Health), With<Health>>,
) {

    for (entity, transform, health) in &health_query {
        if health.hp <= 0.0 {
            commands.entity(entity).despawn();
            let texture: Handle<Image> = asset_serv.load("items/power_up.png");
            commands.spawn((
                Sprite::from_image(texture),
                Transform::from_translation(transform.translation),
                PowerUp
            ));
        }
    }
}

fn update_health_ui(
    player_query: Single<&Health, With<Player>>, 
    mut text_query: Single<&mut Text, With<PlayerHealthText>>,
) {
    text_query.0 = format!("HP: {:.0}", player_query.hp);
}

fn update_damage_ui(
    player_query: Single<&Damage, With<Player>>, 
    mut text_query: Single<&mut Text, With<PlayerDamageText>>,
) {
    text_query.0 = format!("Damage: {:.0}", player_query.damage);
}
 
#[derive(Component)]
struct Player {
    last_hit: f32,
}

#[derive(Component)]
struct PlayerHealthText;

#[derive(Component)]
struct PlayerDamageText;

#[derive(Component)]
struct Projectile {
    direction: Vec2,
    speed: f32,
}

#[derive(Component)]
struct PowerUp;

#[derive(Component)]
struct EnemyProjectile {
    velocity: Vec2,
}

#[derive(Component)]
struct EnemyMovement {
    spawn_time: f32,
    direction: f32, // 1.0 pour la droite vers la gauche, -1.0 pour l'inverse
}

#[derive(Component)]
struct Enemy {
    variety: char,
}

#[derive(Component, Debug, Deserialize)]
struct Health {
    hp: f32
}

#[derive(Component)]
struct Damage {
    damage: f32
}

#[derive(Deserialize, Debug)]
struct  EnemyWave {
    spawn_time: f32,
    pos: Vec2,
    direction: f32,
    hp: Health,
    variety: char
}


#[derive(Deserialize, Asset, TypePath, Debug)]
pub struct LevelData {
    waves: Vec<EnemyWave>
}

#[derive(Resource)]
struct LevelHandle(Handle<LevelData>);

#[derive(Default)]
pub struct LevelDataLoader;

impl AssetLoader for LevelDataLoader {
    type Asset = LevelData;
    type Settings = ();
    type Error = Box<dyn std::error::Error + Send + Sync>;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let level = ron::de::from_bytes::<LevelData>(&bytes)?;
        Ok(level)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}