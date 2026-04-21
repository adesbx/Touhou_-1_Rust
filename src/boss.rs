use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::constants::*;
use bevy::audio::Volume;

pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            move_boss, 
            confine_boss_movement,
            show_health_bar, 
            update_health_bar, 
            delete_health_bar, 
            check_mid_hp,
            basic_shoot_projectiles,
            update_vortex,
            spawn_boss_rain,
            change_attack_type,
            spawn_boss_diagonal_attack
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
        for entity in boss_query {
            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    Sprite::from_color(
                        Color::srgb(0.8, 0.1, 0.1),
                        Vec2::new(20.0, 4.0),
                    ),
                    Transform::from_xyz(0.0, 20.0, 1.0),
                    BossHealthBar,
                ));
            });
        }
    }
}

pub fn update_health_bar(
    mut boss_query: Query<&mut Health, With<Boss>>,
    bar_query: Query<(&mut Sprite, &mut Transform), With<BossHealthBar>>, 
) {
    let max_hp = BOSS_HP;
    let initial_width = 20.0;
    let initial_height: f32 = 4.0;

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
    mut manager: ResMut<LevelManager>, 
) {
    for (health, mut boss, mut transform) in boss_query {
        if health.hp <= BOSS_HP / 2.0 && boss.first_spawn {
            boss.stop_normal_move = true;
            transform.translation.y += 200.0 * time.delta_secs();

            manager.current_phase = GamePhase::PostBoss;
            manager.phase_timer = 0.0;
            manager.next_index = 0;
        } else if health.hp <= BOSS_HP / 2.0 && !boss.first_spawn && boss.phase == 1 {
            boss.phase = 2;
            boss.next_position = Vec3::new(1.0, 0.0, 0.0);
        }
    }
}

pub fn update_vortex(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
    mother_query: Query<(Entity, &Transform, &BasicProjectileBoss), Without<VortexFragment>>,
    mut fragment_query: Query<(&mut Transform, &mut VortexFragment, Entity)>,
) {
    let dt = time.delta_secs();

    for (entity, transform, special) in &mother_query {
        let current_pos = transform.translation.truncate();
        if current_pos.distance(special.start_pos) >= special.explosion_dist {
            commands.entity(entity).despawn();

            let center = transform.translation.truncate();
            let num_fragments = 12;

            commands.spawn((
                AudioPlayer::new(assets.vortex_explosion.clone()),
                PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Despawn,
                    volume: Volume::Decibels(0.2),
                    ..default()
                },
            ));
                
            for i in 0..num_fragments {
                let start_angle = (i as f32) * (std::f32::consts::TAU / num_fragments as f32);
                
                commands.spawn((
                    Sprite::from_image(asset_server.load("projectiles/projectile_vortex_fragment.png")),
                    Transform::from_translation(transform.translation),
                    VortexFragment {
                        center,
                        angle: start_angle,
                        radius: 0.0,
                        rotate_speed: 4.0, 
                        expand_speed: 100.0, 
                    },
                    EnemyProjectile {
                        direction: Vec2::new(0.0, -1.0),
                        speed: BOSS_VORTEX_SPEED
                    },
                ));
            }
        }
    }

    for (mut transform, mut frag, entity) in &mut fragment_query {
        frag.angle += frag.rotate_speed * dt;
        
        frag.radius += frag.expand_speed * dt;

        let max_radius = 75.0; 
        if frag.radius > max_radius {
            commands.entity(entity).despawn();
            continue;
        }

        let new_x = frag.center.x + frag.angle.cos() * frag.radius;
        let new_y = frag.center.y + frag.angle.sin() * frag.radius;
        
        transform.translation = Vec3::new(new_x, new_y, 2.0);
    }
}

fn basic_shoot_projectiles(
    time: Res<Time>,
    mut commands: Commands,
    asset_serv: Res<AssetServer>,
    mut boss_query: Single<(&Transform, &mut Boss), With<Boss>>,
) {
    let (transform, boss) = &mut *boss_query;
    if boss.current_attack == 1 && boss.phase == 1 {
        boss.basic_shoot_timer.tick(time.delta());
        let number_projectile = 4;

        if  boss.basic_shoot_timer.just_finished() {
            let mut rng = rand::thread_rng();

            commands.spawn((
                Sprite::from_image(asset_serv.load("projectiles/projectile_vortex.png")),
                Transform::from_translation(transform.translation), 
                EnemyProjectile {
                    direction: Vec2::new(0.0, -1.0),
                    speed: BOSS_VORTEX_SPEED,
                },
                BasicProjectileBoss {
                    start_pos: transform.translation.truncate(),
                    explosion_dist: rng.gen_range(100.0..300.0),
                },
            ));

            //right
            for i in 1..number_projectile {
                commands.spawn((
                    Sprite::from_image(asset_serv.load("projectiles/projectile_vortex.png")),
                    Transform::from_translation(transform.translation), 
                    EnemyProjectile {
                        direction : Vec2::new(0.19*i as f32 , -1.0),
                        speed: BOSS_VORTEX_SPEED,
                    },
                ));
            }

            //left
            for i in 1..number_projectile {
                commands.spawn((
                    Sprite::from_image(asset_serv.load("projectiles/projectile_vortex.png")),
                    Transform::from_translation(transform.translation), 
                    EnemyProjectile {
                        direction : Vec2::new(-0.19*i as f32 , -1.0),
                        speed: BOSS_VORTEX_SPEED,
                    },
                ));
            }

        }
    }
}

pub fn spawn_boss_rain(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
    mut boss_query: Query<(&Transform, &mut Boss)>,
) {
    let half_width = GAME_WIDTH / 2.0;
    let half_height = GAME_HEIGHT / 2.0;
    let margin = 20.0; // marge pour être sûr d'être à l'intérieur
    
    let x_min = -half_width + margin;
    let x_max = half_width - margin;
    let y_max = half_height - margin;

    for (transform, mut boss) in &mut boss_query {
        if boss.current_attack == 2 && boss.phase == 1 {  

            boss.rain_shoot_timer.tick(time.delta());

            if boss.rain_shoot_timer.just_finished() {
                commands.spawn((
                    AudioPlayer::new(assets.cross_electricity.clone()),
                    PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Despawn,
                        volume: Volume::Decibels(-1.0),
                        ..default()
                    },
                ));
                let mut rng = rand::thread_rng();
                let bullet_count = 15; 
                let texture = asset_server.load("projectiles/projectile_cross_electrised.png");

                for i in 0..bullet_count {
                    // réparti largeur autorisée (entre x_min et x_max)
                    let x_pos = x_min + (i as f32 * (x_max - x_min) / (bullet_count - 1) as f32);
                    
                    let y_spawn = y_max - rng.gen_range(0.0..100.0);

                    let speed = rng.gen_range(150.0..250.0);

                    commands.spawn((
                        Sprite::from_image(texture.clone()),
                        Transform::from_xyz(x_pos, y_spawn, transform.translation.z),
                        EnemyProjectile {
                            direction: Vec2::new(0.0, -1.0), 
                            speed: speed,
                        },
                    ));
                }
            }
        }
    }
}

pub fn spawn_boss_diagonal_attack(
    mut commands: Commands,
    mut boss_query: Single<(&Transform, &mut Boss), With<Boss>>,
    asset_serv: Res<AssetServer>,
    time: Res<Time>,
) {
    let (transform, boss) = &mut *boss_query;
    if boss.current_attack == 1 && boss.phase == 2 {
        boss.diagonal_attack_timer.tick(time.delta());
        
        if boss.diagonal_attack_timer.just_finished() {
            let spacing = 45.0; 
            let start_x = -(GAME_WIDTH / 2.0);
            let end_x = GAME_WIDTH / 2.0;
            let start_y = (GAME_HEIGHT / 2.0) - 50.0;
            let thickness = 5; 
            let now = time.elapsed_secs();

            let safe_limit_x = start_x + (GAME_WIDTH * 0.3);

            let num_cols = ((end_x - start_x) / spacing).floor() as i32;

            for i in 0..num_cols {
                let x = start_x + (i as f32 * spacing);
                
                if x < safe_limit_x { continue; }

                let column_delay = i as f32 * 0.15; 

                for row in 0..thickness {
                    let y = start_y - (row as f32 * spacing);

                    commands.spawn((
                        Sprite::from_image(asset_serv.load("projectiles/projectile.png")),
                        Transform::from_translation(Vec3::new(x, y, -10.0)),
                        EnemyProjectile {
                            speed: 0.0,
                            direction: Vec2::ZERO,
                        },
                        DiagonalMovement {
                            spawn_time: now + column_delay,
                        },
                    ));
                }
            }
        }
    }
}

pub fn change_attack_type(
    time: Res<Time>,
    mut boss_query: Single<&mut Boss, With<Boss>>,
) {
    boss_query.attack_switch_timer.tick(time.delta());

    if boss_query.attack_switch_timer.just_finished() {
        if boss_query.current_attack == 1 {
            boss_query.current_attack = 2;
        } else { 
            boss_query.current_attack = 1;
        }
    }
}