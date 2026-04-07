use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::constants::*;

pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            move_boss, 
            confine_boss_movement,
            show_health_bar, 
            update_health_bar, 
            delete_health_bar, 
            check_mid_hp
        ));
    }
}

pub fn move_boss(
    time: Res<Time>,
    boss_query: Query<(&mut Boss, &mut Transform), With<Boss>>,
) {
    for (mut boss, mut transform)in boss_query {
        if boss.stop_normal_move { continue; }
        if boss.phase == 1 {
            let distance = transform.translation.distance(boss.next_position);
            if  distance < 5.0 {
                boss.next_movement_timer.tick(time.delta());

                if boss.next_movement_timer.just_finished() {
                    let mut rng = rand::thread_rng();

                    boss.next_position = match rng.gen_range(1..7) {
                        1 => Vec3::new(-140.0, 180.0, 2.0),
                        2 => Vec3::new(0.0, 180.0, 2.0),
                        3 => Vec3::new(140.0, 180.0, 2.0),
                        4 => Vec3::new(-140.0, 80.0, 2.0),
                        5 => Vec3::new(0.0, 80.0, 2.0),
                        6 => Vec3::new(140.0, 80.0, 2.0),
                        _=> boss.next_position,
                    };
                }
            } else {

                let direction: Vec3 = (boss.next_position - transform.translation).normalize();
                            
                let velocity = direction * BOSS_SPEED * time.delta_secs();
                
                // Pour éviter de dépasser la cible (ce qui crée des vibrations)
                if velocity.length() > distance {
                    transform.translation = boss.next_position;
                } else {
                    transform.translation += velocity;
                }
            }
        } else if boss.phase == 2 {
            let half_enemy_size: f32 = ANGEL_SIZE / 2.0;
            let half_width: f32 = GAME_WIDTH / 2.0;
            let x_min = -half_width + half_enemy_size;
            let x_max = half_width - half_enemy_size;

            if transform.translation.x >= x_max {
                boss.next_position = Vec3::new(-1.0, 0.0, 0.0);
            } else if transform.translation.x <= x_min  {
                boss.next_position = Vec3::new(1.0, 0.0, 0.0);
            }
            let velocity =  boss.next_position * (BOSS_SPEED/2.0) * time.delta_secs();
            transform.translation += velocity;
        }
    }
}

fn confine_boss_movement(    
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Boss>>,

) {
    let half_enemy_size: f32 = BOSS_SIZE / 2.0;
    let half_height: f32 = GAME_HEIGHT / 2.0;


    let y_max = half_height + half_enemy_size + 100.0;

    for (entity, transform) in &enemy_query {
        if transform.translation.y > y_max {
            commands.entity(entity).despawn();
        }
    }
}

pub fn show_health_bar(
    mut commands: Commands,
    boss_query: Query<Entity, With<Boss>>,
    bar_query: Query<Entity, With<BossHealthBar>>, 
) {
    if !boss_query.is_empty() && bar_query.is_empty() {
        commands.spawn((
            Sprite::from_color(
                Color::srgb(0.8, 0.1, 0.1),
                Vec2::new(300.0, 20.0),
            ),
            Transform::from_xyz(0.0, -GAME_HEIGHT / 2.0 + 30.0, 20.0),
            BossHealthBar,
        ));
    }
}

pub fn update_health_bar(
    mut boss_query: Query<&mut Health, With<Boss>>,
    bar_query: Query<(&mut Sprite, &mut Transform), With<BossHealthBar>>, 
) {
    let max_hp = BOSS_HP;
    let initial_width = 300.0;
    let initial_height: f32 = 20.0;

    if !boss_query.is_empty() && !bar_query.is_empty() {
            for (mut sprite, mut transform) in bar_query {
                for health in &mut boss_query {
                    let current_width = health.hp / max_hp * initial_width;
                    sprite.custom_size = Some(Vec2::new(current_width, initial_height));

                    let offset_x = (initial_width - current_width) / 2.0;
        
                    transform.translation.x = offset_x;
                }
            }
    }
}

pub fn delete_health_bar(
    mut commands: Commands,
    boss_query: Query<&mut Health, With<Boss>>,
    bar_query: Query<Entity, With<BossHealthBar>>, 
) {
    if boss_query.is_empty() && !bar_query.is_empty() {
        for entity in bar_query {
            commands.entity(entity).despawn();
        }
    }
}

pub fn check_mid_hp(
    time: Res<Time>,
    boss_query: Query<(&mut Health, &mut Boss, &mut Transform), With<Boss>>,
) {
    for (health, mut boss, mut transform) in boss_query {
        if health.hp <= BOSS_HP / 2.0 && boss.first_spawn{
            boss.stop_normal_move = true;
            transform.translation.y += 200.0 * time.delta_secs();
        } else if health.hp <= BOSS_HP / 2.0 && !boss.first_spawn && boss.phase == 1 {
            boss.phase = 2;
            boss.next_position = Vec3::new(1.0, 0.0, 0.0);
        }
    }
}