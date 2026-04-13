use bevy::audio::Volume;
use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            shoot_projectile, 
            move_projectile, 
            confine_projectile_movement, 
            check_collison_projectile_player, 
            enemies_shoot_projectiles, 
            move_enemy_projectiles
        ));
    }
}

fn shoot_projectile(
    time:  Res<Time>,
    mut commands: Commands,
    asset_serv: Res<AssetServer>,
    assets: Res<GameAssets>,
    mut player_query: Single<(&Transform, &mut Damage, &mut Player), With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let (transform, damage_player, player) = &mut *player_query;

    player.shoot_timer.tick(time.delta());
    player.shoot_timer_fire.tick(time.delta());

    if keyboard.pressed(KeyCode::KeyK) && player.shoot_timer.is_finished(){
        let texture = asset_serv.load("projectiles/projectile.png");
        let fire_texture = asset_serv.load("projectiles/fire_ball.png");


        let base_x = transform.translation.x;
        let base_y = transform.translation.y + 10.0;
        let z = transform.translation.z;

        commands.spawn((
            AudioPlayer::new(assets.shoot_sound.clone()),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: Volume::Decibels(-0.4),
                ..default()
            },
        ));

        if damage_player.damage < 50.0 {
            commands.spawn((
                Sprite::from_image(texture),
                Transform::from_xyz(
                    base_x,
                    base_y,
                    z
                ),
                Projectile { direction: Vec2::new(0.0, 1.0), speed: PROJECTILE_SPEED, variety: 'b', spawn_time: time.elapsed_secs()},
            ));
        } else if damage_player.damage < 100.0 {
            commands.spawn((
                Sprite::from_image(texture.clone()),
                Transform::from_xyz(base_x, base_y, z),
                Projectile { direction: Vec2::new(0.0, 1.0), speed: PROJECTILE_SPEED, variety: 'b', spawn_time: time.elapsed_secs(),},
            ));

            commands.spawn((
                Sprite::from_image(texture.clone()),
                Transform::from_xyz(base_x, base_y, z),
                Projectile { direction: Vec2::new(-0.5, 1.0).normalize(), speed: PROJECTILE_SPEED, variety: 'b', spawn_time: time.elapsed_secs()},
            ));

            commands.spawn((
                Sprite::from_image(texture),
                Transform::from_xyz(base_x, base_y, z),
                Projectile { direction: Vec2::new(0.5, 1.0).normalize(), speed: PROJECTILE_SPEED, variety: 'b', spawn_time: time.elapsed_secs()},
            ));
        } else if damage_player.damage < 150.0 {
                        commands.spawn((
                Sprite::from_image(texture.clone()),
                Transform::from_xyz(base_x, base_y, z),
                Projectile { direction: Vec2::new(0.0, 1.0), speed: PROJECTILE_SPEED, variety: 'b', spawn_time: time.elapsed_secs()},
            ));

            commands.spawn((
                Sprite::from_image(texture.clone()),
                Transform::from_xyz(base_x, base_y, z),
                Projectile { direction: Vec2::new(-0.5, 1.0).normalize(), speed: PROJECTILE_SPEED, variety: 'b', spawn_time: time.elapsed_secs()},
            ));

            commands.spawn((
                Sprite::from_image(texture),
                Transform::from_xyz(base_x, base_y, z),
                Projectile { direction: Vec2::new(0.5, 1.0).normalize(), speed: PROJECTILE_SPEED, variety: 'b', spawn_time: time.elapsed_secs()},
            ));
            
            if  player.shoot_timer_fire.is_finished() {
                if player.shoot_from_left {
                commands.spawn((
                        Sprite::from_image(fire_texture),
                        Transform::from_xyz(base_x-10.0, base_y, z),
                        Projectile { direction: Vec2::new(1.0, 1.0).normalize(), speed: PROJECTILE_SPEED, variety: 'f', spawn_time: time.elapsed_secs()},
                    ));
                } else {
                    commands.spawn((
                        Sprite::from_image(fire_texture),
                        Transform::from_xyz(base_x+10.0, base_y, z),
                        Projectile { direction: Vec2::new(-1.0, 1.0).normalize(), speed: PROJECTILE_SPEED, variety: 'f', spawn_time: time.elapsed_secs()},
                    ));
                }
                player.shoot_from_left = !player.shoot_from_left
            }
        }

    }
}


fn move_projectile(
    time:  Res<Time>,
    mut projectile_query: Query<(&mut Transform, &Projectile)>,
) {
    let elapsed = time.elapsed_secs();
    let dt = time.delta_secs();

    for (mut transform, projectile) in &mut projectile_query {

        if projectile.variety == 'b' {// projectile basique fonce dans sa direction
            let movement = projectile.direction.normalize() * projectile.speed * time.delta_secs();
            transform.translation += movement.extend(0.0);
        } else if projectile.variety == 'f' {
            let local_time: f32 = elapsed - projectile.spawn_time;
            transform.translation.y += projectile.speed * dt;
            let x_force = local_time * local_time * 300.0;
            transform.translation.x += x_force * projectile.direction.x * dt;
        }
    }
}

fn confine_projectile_movement(    
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    enemy_projectile_query: Query<(Entity, &Transform), With<EnemyProjectile>>,

) {
    let half_projectile_size: f32 = PROJECTILE_SIZE / 2.0;
    let half_cross_projectile_size: f32 = CROSS_PROJECTILE_SIZE / 2.0;
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

    let x_min = -half_width + half_cross_projectile_size;
    let x_max = half_width - half_cross_projectile_size;
    let y_min = -half_height + half_cross_projectile_size;
    let y_max = half_height - half_cross_projectile_size;

    for (entity, transform) in &enemy_projectile_query {
        if transform.translation.y > y_max || transform.translation.y < y_min || transform.translation.x > x_max || transform.translation.x < x_min {
            commands.entity(entity).despawn();
        }
    }
}


fn check_collison_projectile_player(
    time: Res<Time>,
    mut commands: Commands,
    enemy_projectile_query: Query<(Entity, &Transform), With<EnemyProjectile>>,
    enemy_query: Query<(Entity, &Transform, &Enemy), With<Enemy>>,
    mut player_query: Single<(&Transform, &mut Health, &mut Player), With<Player>>,
) {
    let (transform, health, player) = &mut *player_query; // possiblement sale voir pour faire autrement
    if time.elapsed_secs() - player.last_hit > INVINCIBILITY_TIME  {
        for (projectile_entity, projectile_transform) in &enemy_projectile_query {
            let p1 = projectile_transform.translation.truncate(); // Vec3 -> Vec2
            let p2 = transform.translation.truncate();
            let distance = p1.distance(p2);
            if distance < (CROSS_PROJECTILE_SIZE + PLAYER_SIZE) / 2.0 {                
                commands.entity(projectile_entity).despawn();
                health.hp -= 1.0;
                player.last_hit = time.elapsed_secs();
                break;
            }
        }

        for (enemy_entity, enemy_transform, enemy) in &enemy_query {
            let p1 = enemy_transform.translation.truncate(); // Vec3 -> Vec2
            let p2 = transform.translation.truncate();
            let distance = p1.distance(p2);

            let mut size = 0.0;
            if enemy.variety == 'c' {
                size = CHERUB_SIZE
            } else if enemy.variety == 'a' {
                size = ANGEL_SIZE
            }

            if distance < (size + PLAYER_SIZE) / 2.0 {                
                commands.entity(enemy_entity).despawn();
                health.hp -= 1.0;
                player.last_hit = time.elapsed_secs();
                break;
            }
        }
    }
}

fn enemies_shoot_projectiles(
    time: Res<Time>,
    mut commands: Commands,
    asset_serv: Res<AssetServer>,
    mut enemy_query: Query<(&Transform, &mut Enemy), With<Enemy>>,
    player_transform: Single<&Transform, With<Player>>,
) {
    let player_pos = player_transform.translation.truncate(); 

    for (transform, mut enemy) in &mut enemy_query {
        
        enemy.shoot_timer.tick(time.delta());
        if enemy.shoot_timer.is_finished() && enemy.variety == 'a' {  
            let enemy_pos = transform.translation.truncate();
            let direction = (player_pos - enemy_pos).normalize_or_zero();

            let texture: Handle<Image> = asset_serv.load("projectiles/projectile_cross.png");
            commands.spawn((
                Sprite::from_image(texture),
                Transform::from_translation(transform.translation),
                EnemyProjectile{
                    direction: direction,
                    speed: CROSS_PROJECTILE_SPEED
                },
            ));
          }
    }
}

fn move_enemy_projectiles(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &EnemyProjectile)>,
) {
    for (mut transform, projectile) in &mut query {
        let movement: Vec2 = projectile.direction.normalize() * projectile.speed * time.delta_secs();
        transform.translation += movement.extend(0.0);
    }
}