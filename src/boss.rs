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
            shoot_boss_rain,
            shoot_boss_diagonal_attack,
            shoot_boomerang_attack,
            change_attack_type,
            update_boss_sprite
        ).run_if(in_state(GameState::Running)));
    }
}

pub fn move_boss(
    time: Res<Time>,
    mut boss_query: Single<(&mut Boss, &mut Transform), With<Boss>>,
) {
    let (boss, transform) = &mut *boss_query;
    if boss.stop_normal_move { return; }
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
        transform.translation.y = 140.0;
    }
}

fn confine_boss_movement(    
    mut commands: Commands,
    mut boss_query: Single<(Entity, &Transform), With<Boss>>,

) {
    let (mut entity, transform) = &mut *boss_query;
    let half_enemy_size: f32 = BOSS_SIZE / 2.0;
    let half_height: f32 = GAME_HEIGHT / 2.0;

    let y_max = half_height + half_enemy_size + 100.0;

    if transform.translation.y > y_max {
        commands.entity(entity).despawn();
    }
}

pub fn show_health_bar(
    mut commands: Commands,
    boss_query: Query<Entity, With<Boss>>,
    bar_query: Query<Entity, With<BossHealthBar>>, 
    asset_server: Res<AssetServer>
) {
    if !boss_query.is_empty() && bar_query.is_empty() {
        for entity in boss_query {
            commands.entity(entity).with_children(|parent| {

                parent.spawn((
                    Sprite::from_image(asset_server.load("hud/boss_bar.png")),
                    Transform::from_xyz(0.0, 20.0, 1.0),
                ));

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

//différent tir de boss
//phase 1

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
                Sprite {
                    image: asset_serv.load("projectiles/projectile_vortex.png"),
                    color: Color::srgb(0.2, 0.6, 0.9),
                    ..default()
                },
                Transform::from_translation(transform.translation), 
                EnemyProjectile {
                    direction: Vec2::new(0.0, -1.0),
                    speed: BOSS_VORTEX_SPEED,
                    size: BOSS_VORTEX_SIZE
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
                        size: BOSS_VORTEX_SIZE
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
                        size: BOSS_VORTEX_SIZE
                    },
                ));
            }

        }
    }
}

pub fn shoot_boss_rain(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
    mut boss_query: Single<(&Transform, &mut Boss), With<Boss>>,
) {
    let (transform, boss) = &mut *boss_query;

    let half_width = GAME_WIDTH / 2.0;
    let half_height = GAME_HEIGHT / 2.0;
    let margin = 20.0; // marge pour être sûr d'être à l'intérieur
    
    let x_min = -half_width + margin;
    let x_max = half_width - margin;
    let y_max = half_height - margin;

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
                        size: BOSS_CROSS_ELECTRISED
                    },
                ));
            }
        }
    }
}

//phase 2

pub fn shoot_boss_diagonal_attack(
    clock: ResMut<GameClock>,
    mut commands: Commands,
    mut boss_query: Single<(&Transform, &mut Boss), With<Boss>>,
    time: Res<Time>,
) {
    let (_, boss) = &mut *boss_query;
    if boss.current_attack == 1 && boss.phase == 2 {
        boss.diagonal_attack_timer.tick(time.delta());
        
        if boss.diagonal_attack_timer.just_finished() {
            let spacing = 30.0; 
            let start_x = -(GAME_WIDTH / 2.0);
            let end_x: f32 = GAME_WIDTH / 2.0;
            let start_y = (GAME_HEIGHT / 2.0) - 50.0;
            let thickness = 50; 
            let now = clock.watch.elapsed_secs();
            let safe_limit_x = start_x + (GAME_WIDTH * 0.1);
            let num_cols = ((end_x - start_x) / spacing).floor() as i32;

            for i in 0..num_cols {
                let x = start_x + (i as f32 * spacing);
                if x < safe_limit_x { continue; }

                for row in 0..thickness {
                    let y = start_y - (row as f32 * spacing);
                    let delay = (i as f32 * 0.15) + (row as f32 * 0.10); 

                    //Créer un spawner pour créer un projectile (dans projectile.rs)
                    commands.spawn((
                        DiagonalMovementSpawner {
                            x: x,
                            y: y,
                            spawn_time: now + delay,
                        },
                    ));
                }
            }
        }
    }
}

pub fn shoot_boomerang_attack(
    mut commands: Commands,
    time: Res<Time>,
    clock: ResMut<GameClock>,
    asset_serv: Res<AssetServer>,
    mut current_angle: Local<f32>,
    mut boss_query: Single<(&Transform, &mut Boss), With<Boss>>,
) {
    let (transform, boss) = &mut *boss_query;
    
    if boss.current_attack == 2 && boss.phase == 2 {

        boss.boomerang_attack_timer.tick(time.delta());
    
        if boss.boomerang_attack_timer.just_finished() {
            let num_projectiles = 16; 
            let num_waves = 5;
            let step = (std::f32::consts::PI * 2.0) / num_projectiles as f32;
            let now = clock.watch.elapsed_secs();

            for w in 0..num_waves{
                let wave_distance = 300.0 - (w as f32 * 50.0);
                let wave_angle_offset = w as f32 * 0.15;

                for i in 0..num_projectiles {
                    let angle = *current_angle + (i as f32 * step) + wave_angle_offset;

                    commands.spawn((
                        Sprite::from_image(asset_serv.load("projectiles/projectile_vortex_fragment.png")),
                        Transform::from_translation(transform.translation),
                        BoomerangProjectile {
                            angle,
                            start_pos: transform.translation,
                            start_time: now,
                            custom_distance: wave_distance
                        },
                        EnemyProjectile { 
                            speed: 0.0, 
                            direction: Vec2::ZERO,
                            size: BOSS_VORTEX_FRAGMENT_SIZE 
                        },
                    ));
                }
            }
            *current_angle += 0.4; // si grand, spirale sérée
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

fn update_boss_sprite(
    time:  Res<Time>,
    boss: Single<(&mut Sprite, &mut Boss), With<Boss>>,
) {

    let (mut sprite, mut boss) = boss.into_inner();

    if let Some(atlas) = sprite.texture_atlas.as_mut() {
        boss.animation_timer.tick(time.delta());

        if boss.animation_timer.just_finished() {
            if atlas.index >= 9 {
                atlas.index = 0;
            } else {
                atlas.index += 1;
            }
        }
    }
}