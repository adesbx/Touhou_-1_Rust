use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::constants::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_enemies, confine_enemies_movement, check_collison_enemies, check_health, update_player_sprites));
    }
}

fn move_enemies(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &EnemyMovement, &Enemy), (With<Enemy>, Without<Player>, Without<Boss>)>,
    player_query: Single<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let dt = time.delta_secs();
    let elapsed = time.elapsed_secs();
    let p1 = player_query.translation.truncate();
    
    let attraction_speed = 150.0;
    let attraction_range = 70.0; 

    for (mut transform, movement, enemy) in &mut query {
        let local_time = elapsed - movement.spawn_time;
        if enemy.variety == 'c' {
            let p2 = transform.translation.truncate();
            let distance = p1.distance(p2);
            if distance < attraction_range {     
                let direction = (p1 - p2).normalize_or_zero();
                let strength = (1.0 - (distance / attraction_range)).max(0.6); //0.6 force minimum d'attraction (rapide ou pas)
                transform.translation += direction.extend(0.0) * attraction_speed * strength * dt;
                continue; // Le Cherub ignore son pattern s'il chasse le joueur
            }
        }

        match movement.pattern {
            MovePattern::Straight => {
                transform.translation.y -= 80.0 * dt;
            }
            
            MovePattern::ZigZag(intensity) => {
                transform.translation.y -= 60.0 * dt;
                let x_move = (local_time * 10.0).sin() * intensity;
                transform.translation.x += x_move * movement.direction * dt;
            }

            MovePattern::SineWave => {
                transform.translation.y -= 50.0 * dt;
                let offset_x = (local_time * 1.5).sin() * 75.0;
                transform.translation.x += offset_x * movement.direction * dt;
            }

            MovePattern::Spiral(curve) => {
                transform.translation.y -= 40.0 * dt;
                transform.translation.x += (local_time * 5.0).cos() * curve;
                transform.translation.y += (local_time * 5.0).sin() * curve;
            }

            MovePattern::Arc(curve_strength) => {
                transform.translation.y -= 70.0 * dt;
                let x_force = local_time * local_time * curve_strength;
                transform.translation.x += x_force * movement.direction * dt;
            }

            MovePattern::StraightPause(pause) => {
                if transform.translation.y > pause {
                    transform.translation.y -= 80.0 * dt;
                }
            }
        }
    }
}

fn confine_enemies_movement(    
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), (With<Enemy>, Without<Boss>)>,

) {
    let half_enemy_size: f32 = ANGEL_SIZE / 2.0;
    let half_height: f32 = GAME_HEIGHT / 2.0;
    let half_width: f32 = GAME_WIDTH / 2.0;


    let x_min = -half_width + half_enemy_size;
    let x_max = half_width - half_enemy_size;
    let y_min = -half_height + half_enemy_size;

    for (entity, transform) in &enemy_query {
        if transform.translation.y < y_min || transform.translation.x > x_max || transform.translation.x < x_min {
            commands.entity(entity).despawn();
        }
    }
}

fn check_collison_enemies(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    mut enemy_query: Query<(&Transform, &mut Health, &Enemy), With<Enemy>>,
) {

    for (projectile_entity, projectile_transform) in &projectile_query {
        for (enemy_transform, mut enemy_health, enemy) in &mut enemy_query {
            let p1 = projectile_transform.translation.truncate(); // Vec3 -> Vec2
            let p2 = enemy_transform.translation.truncate();
            let distance = p1.distance(p2);
            let mut size = 0.0;
            if enemy.variety == 'c' {
                size = CHERUB_SIZE
            } else if enemy.variety == 'a' {
                size = ANGEL_SIZE
            }
            
            if distance < (PROJECTILE_SIZE + size) / 2.0 {                
                commands.entity(projectile_entity).despawn();
                enemy_health.hp -= PLAYER_DAMAGE; //anciennement player_query.damage; mtn les ennemient perdent un montant fixe par projectile
                break;
            }
        }
    }
}

//marche également pour le joueur
fn check_health(
    mut commands: Commands,
    asset_serv: Res<AssetServer>,
    health_query: Query<(Entity, &Transform ,&Health), With<Health>>,
) {

    for (entity, transform, health) in &health_query {
        if health.hp <= 0.0 {
            commands.entity(entity).despawn();
            let mut rng = rand::thread_rng();
            if rng.gen_range(1..4) == 1 { // 1/3 de faire spawbn un power up  
                let texture: Handle<Image> = asset_serv.load("items/power_up.png");
                commands.spawn((
                    Sprite::from_image(texture),
                    Transform::from_translation(transform.translation),
                    PowerUp
                ));
            }
        }
    }
}

fn update_player_sprites(
    time:  Res<Time>,
    mut enemies: Query<(&mut Sprite, &mut Enemy), With<Enemy>>,
) {
    for (mut sprite, mut enemy) in &mut enemies {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            enemy.animation_timer.tick(time.delta());

            if enemy.animation_timer.just_finished() {
                if atlas.index >=1 {
                    atlas.index = 0
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}