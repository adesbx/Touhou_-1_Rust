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
            move_enemy_projectiles,
            move_diagonal_projectiles,
            update_diagonal_sprites,
            update_vortex,
            move_boomerang_projectiles,
        ).run_if(in_state(GameState::Running)));
    }
}

fn shoot_projectile(
    time:  Res<Time>,
    clock: ResMut<GameClock>,
    mut commands: Commands,
    asset_serv: Res<AssetServer>,
    assets: Res<GameAssets>,
    mut player_query: Single<(&Transform, &mut Damage, &mut Player), With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut last_sound_time: Local<f32>,
) {
    let (transform, damage_player, player) = &mut *player_query;

    player.shoot_timer.tick(time.delta());
    player.shoot_timer_fire.tick(time.delta());

    if keyboard.pressed(KeyCode::KeyK) && player.shoot_timer.is_finished(){
        let base_x = transform.translation.x;
        let base_y = transform.translation.y + 10.0;
        let z = transform.translation.z;

        if clock.watch.elapsed_secs() - *last_sound_time > 0.1 {
            commands.spawn((
                AudioPlayer::new(assets.shoot_sound.clone()),
                PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Despawn,
                    volume: Volume::Decibels(-10.0),
                    ..default()
                },
            ));
            *last_sound_time = clock.watch.elapsed_secs();
        }

        if damage_player.damage < 75.0 {
            first_power_shooting(&clock, &mut commands, &asset_serv, base_x, base_y, z);
        } else if damage_player.damage < 150.0 {
            second_power_shooting(&clock, &mut commands, &asset_serv, base_x, base_y, z);
        } else if damage_player.damage < 250.0 {
            third_power_shooting(&clock, &mut commands, &asset_serv, &assets, player, base_x, base_y, z);
        } else {
            fourth_power_shooting(&clock, &mut commands, &asset_serv, &assets, player, base_x, base_y, z);
        }
    }
}

fn first_power_shooting(
    clock: &GameClock,
    commands: &mut Commands,
    asset_serv: &AssetServer,
    base_x: f32,
    base_y: f32,
    base_z: f32
) {
    commands.spawn((
        Sprite::from_image(asset_serv.load("projectiles/projectile.png")),
        Transform::from_xyz(
            base_x,
            base_y,
            base_z
        ),
        Projectile { direction: Vec2::new(0.0, 1.0), speed: PROJECTILE_SPEED, variety: 'b', spawn_time: clock.watch.elapsed_secs()},
    ));
}

fn second_power_shooting(
    clock: &GameClock,
    commands: &mut Commands,
    asset_serv: &AssetServer,
    base_x: f32,
    base_y: f32,
    base_z: f32
) {
    let texture = asset_serv.load("projectiles/projectile.png");
    commands.spawn((
        Sprite::from_image(texture.clone()),
        Transform::from_xyz(base_x, base_y, base_z),
        Projectile { direction: Vec2::new(0.0, 1.0), speed: PROJECTILE_SPEED, variety: 'b', spawn_time: clock.watch.elapsed_secs(),},
    ));

    commands.spawn((
        Sprite::from_image(texture.clone()),
        Transform::from_xyz(base_x, base_y, base_z),
        Projectile { direction: Vec2::new(-0.25, 1.0).normalize(), speed: PROJECTILE_SPEED, variety: 'b', spawn_time: clock.watch.elapsed_secs()},
    ));

    commands.spawn((
        Sprite::from_image(texture),
        Transform::from_xyz(base_x, base_y, base_z),
        Projectile { direction: Vec2::new(0.25, 1.0).normalize(), speed: PROJECTILE_SPEED, variety: 'b', spawn_time: clock.watch.elapsed_secs()},
    ));
}

fn third_power_shooting(
    clock: &GameClock,
    commands: &mut Commands,
    asset_serv: &AssetServer,
    assets: &GameAssets,
    player: &mut Mut<Player>,
    base_x: f32,
    base_y: f32,
    base_z: f32
) {
    let fire_texture = asset_serv.load("projectiles/fire_ball.png");
    second_power_shooting(clock, commands, asset_serv, base_x, base_y, base_z);
    
    if  player.shoot_timer_fire.is_finished() {
        commands.spawn((
            AudioPlayer::new(assets.shoot_fire_sound.clone()),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: Volume::Decibels(-8.0),
                ..default()
            },
        ));
        if player.shoot_from_left {
        commands.spawn((
                Sprite {
                    image: fire_texture,
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..default()
                },
                Transform::from_xyz(base_x-10.0, base_y, base_z),
                Projectile { direction: Vec2::new(1.0, 1.0).normalize(), speed: PROJECTILE_SPEED, variety: 'f', spawn_time: clock.watch.elapsed_secs()},
            ));
        } else {
            commands.spawn((
                Sprite {
                    image: fire_texture,
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..default()
                },
                Transform::from_xyz(base_x+10.0, base_y, base_z),
                Projectile { direction: Vec2::new(-1.0, 1.0).normalize(), speed: PROJECTILE_SPEED, variety: 'f', spawn_time: clock.watch.elapsed_secs()},
            ));
        }
        player.shoot_from_left = !player.shoot_from_left
    }
}


fn fourth_power_shooting(
    clock: &GameClock,
    commands: &mut Commands,
    asset_serv: &AssetServer,
    assets: &GameAssets,
    player: &mut Mut<Player>,
    base_x: f32,
    base_y: f32,
    base_z: f32
) {
    let fire_texture = asset_serv.load("projectiles/fire_ball.png");
    second_power_shooting(clock, commands, asset_serv, base_x, base_y, base_z);
    
    if  player.shoot_timer_fire.is_finished() {
        commands.spawn((
            AudioPlayer::new(assets.shoot_fire_sound.clone()),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: Volume::Decibels(-8.0),
                ..default()
            },
        ));
        //tire des deux cotés mtn
        commands.spawn((
                Sprite {
                    image: fire_texture.clone(),
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..default()
                },
                Transform::from_xyz(base_x-10.0, base_y, base_z),
                Projectile { direction: Vec2::new(1.0, 1.0).normalize(), speed: PROJECTILE_SPEED, variety: 't', spawn_time: clock.watch.elapsed_secs()},
            ));
        commands.spawn((
            Sprite {
                image: fire_texture.clone(),
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            Transform::from_xyz(base_x+10.0, base_y, base_z),
            Projectile { direction: Vec2::new(-1.0, 1.0).normalize(), speed: PROJECTILE_SPEED, variety: 't', spawn_time: clock.watch.elapsed_secs()},
        ));
    }
}

fn move_projectile(
    time:  Res<Time>,
    clock: ResMut<GameClock>,
    mut projectile_query: Query<(&mut Transform, &Projectile), Without<Enemy>>,
    enemy_query: Query<(&Transform, &Enemy), (With<Enemy>, Without<Boss>)>,
) {
    let elapsed = clock.watch.elapsed_secs();
    let dt = time.delta_secs();

    for (mut transform, projectile) in &mut projectile_query {

        if projectile.variety == 'b' {// projectile basique fonce dans sa direction
            let movement = projectile.direction.normalize() * projectile.speed * dt;
            transform.translation += movement.extend(0.0);
        } else if projectile.variety == 'f' { // feu basique
            let local_time: f32 = elapsed - projectile.spawn_time;
            transform.translation.y += projectile.speed * dt;
            let x_force = local_time * local_time * 300.0;
            transform.translation.x += x_force * projectile.direction.x * dt;
        } else if projectile.variety == 't' { //feu homing tear
            let local_time: f32 = elapsed - projectile.spawn_time;
            let mut closest_enemy = Vec3::ZERO;
            let mut min_distance = f32::MAX; 
            for (enemy_transform, _) in &enemy_query {
                let distance = transform.translation.distance(enemy_transform.translation);
                if min_distance > distance {
                    min_distance = distance;
                    closest_enemy = enemy_transform.translation;
                }
            }

            if closest_enemy != Vec3::ZERO {
                let direction = (closest_enemy - transform.translation).normalize();
                transform.translation += direction * projectile.speed * dt;
            } else {
                transform.translation.y += projectile.speed * dt;
            }

            let x_force = local_time * local_time * 300.0;
            
            transform.translation.x += x_force * projectile.direction.x * dt;
        }
    }
}

pub fn move_diagonal_projectiles(
    mut commands: Commands,
    clock: ResMut<GameClock>,
    mut spawner_query: Query<(Entity, &DiagonalMovementSpawner)>,
    mut despawner_query: Query<(Entity, &DiagonalMovementDespawner)>,
    asset_serv: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    let now = clock.watch.elapsed_secs();
    let lifetime = 2.0; 

    for (entity, movement) in &mut spawner_query {
        if now >= movement.spawn_time {
                
            let texture = asset_serv.load("projectiles/projectile_diagonal_2.png");
            let layout = TextureAtlasLayout::from_grid(UVec2::splat(20), 2, 3, None, None);
            let texture_atlas_layout = texture_atlas_layout.add(layout);        
            commands.spawn((
                Sprite {
                    image:texture, 
                    texture_atlas: Some(TextureAtlas { layout: texture_atlas_layout, index: 0}),
                    custom_size: Some(Vec2::new(28.0, 28.0)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(movement.x, movement.y, 10.0)),
                EnemyProjectile {
                    speed: 0.0,
                    direction: Vec2::ZERO,
                },
                DiagonalMovementDespawner {
                    spawn_time: now,
                    animation_timer: Timer::from_seconds(0.4, TimerMode::Repeating),
                },
            ));
            commands.entity(entity).despawn();
        }
    }

    for (entity, movement) in &mut despawner_query {
        if now >= movement.spawn_time + lifetime {
            commands.entity(entity).despawn();
        }
    }
}

fn confine_projectile_movement(    
    mut commands: Commands,
    mut set: ParamSet<(
        Query<(Entity, &Transform), With<Projectile>>,
        Query<(Entity, &Transform), (With<EnemyProjectile>, Without<BoomerangProjectile>)>,
        Query<&mut Transform, With<BoomerangProjectile>>,
    )>,
) {
    let half_height: f32 = GAME_HEIGHT / 2.0;
    let half_width: f32 = GAME_WIDTH / 2.0;

    let x_min_p = -half_width + (PROJECTILE_SIZE / 2.0);
    let x_max_p = half_width - (PROJECTILE_SIZE / 2.0);
    let y_min_p = -half_height + (PROJECTILE_SIZE / 2.0);
    let y_max_p = half_height - (PROJECTILE_SIZE / 2.0);

    for (entity, transform) in set.p0().iter() {
        if transform.translation.y > y_max_p || transform.translation.y < y_min_p || 
           transform.translation.x > x_max_p || transform.translation.x < x_min_p {
            commands.entity(entity).despawn();
        }
    }

    for mut transform in set.p2().iter_mut() {
        if transform.translation.y > y_max_p || transform.translation.y < y_min_p || 
           transform.translation.x > x_max_p || transform.translation.x < x_min_p {
            transform.translation.z = -1500.0; 
        } else {
            transform.translation.z = 15.0;
        }
    }

    let x_min_e = -half_width + (CROSS_PROJECTILE_SIZE / 2.0);
    let x_max_e = half_width - (CROSS_PROJECTILE_SIZE / 2.0);
    let y_min_e = -half_height + (CROSS_PROJECTILE_SIZE / 2.0);
    let y_max_e = half_height - (CROSS_PROJECTILE_SIZE / 2.0);

    for (entity, transform) in set.p1().iter() {
        if transform.translation.y > y_max_e || transform.translation.y < y_min_e || 
           transform.translation.x > x_max_e || transform.translation.x < x_min_e {
            commands.entity(entity).despawn();
        }
    }
}


fn check_collison_projectile_player(
    clock: ResMut<GameClock>,
    mut commands: Commands,
    enemy_projectile_query: Query<(Entity, &Transform), With<EnemyProjectile>>,
    enemy_query: Query<(Entity, &Transform, &Enemy), With<Enemy>>,
    mut player_query: Single<(&Transform, &mut Health, &mut Player), With<Player>>,
) {
    let (transform, health, player) = &mut *player_query; // possiblement sale voir pour faire autrement
    if clock.watch.elapsed_secs() - player.last_hit > INVINCIBILITY_TIME  {
        for (projectile_entity, projectile_transform) in &enemy_projectile_query {
            let p1 = projectile_transform.translation.truncate(); // Vec3 -> Vec2
            let p2 = transform.translation.truncate();
            let distance = p1.distance(p2);
            if distance < (CROSS_PROJECTILE_SIZE + PLAYER_SIZE) / 2.0 {                
                commands.entity(projectile_entity).despawn();
                health.hp -= 1.0;
                player.last_hit = clock.watch.elapsed_secs();
                break;
            }
        }

        for (enemy_entity, enemy_transform, enemy) in &enemy_query {
            let p1 = enemy_transform.translation.truncate(); // Vec3 -> Vec2
            let p2 = transform.translation.truncate();
            let distance = p1.distance(p2);

            let mut size = 0.0;
            if enemy.variety == 'c' {
                size = CHERUB_SIZE;
            } else if enemy.variety == 'a' {
                size = ANGEL_SIZE;
            } else if enemy.variety == 'b' {
                size = BOSS_SIZE;
            }

            if distance < (size + PLAYER_SIZE) / 2.0 && enemy.variety != 'b' {                
                commands.entity(enemy_entity).despawn();
                health.hp -= 1.0;
                player.last_hit = clock.watch.elapsed_secs();
                break;
            } else if distance < (size + PLAYER_SIZE) / 2.0 && enemy.variety == 'b'{
                health.hp -= 1.0;
                player.last_hit = clock.watch.elapsed_secs();
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

        // TODO : Voir si laisser son tir ennemi
        // commands.spawn((
        //     AudioPlayer::new(assets.enemy_shoot_sound.clone()),
        //     PlaybackSettings {
        //         mode: bevy::audio::PlaybackMode::Despawn,
        //         volume: Volume::Decibels(-15.0),
        //         ..default()
        //     },
        // ));

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
    mut query: Query<(&mut Transform, &EnemyProjectile), (Without<DiagonalMovementDespawner>, Without<BoomerangProjectile>)>,
) {
    for (mut transform, projectile) in &mut query {
        let movement: Vec2 = projectile.direction.normalize() * projectile.speed * time.delta_secs();
        transform.translation += movement.extend(0.0);
    }
}

// Boss projectile

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

pub fn move_boomerang_projectiles(
    mut commands: Commands,
    clock: ResMut<GameClock>,
    mut query: Query<(Entity, &mut Transform, &BoomerangProjectile)>,
) {
    let now = clock.watch.elapsed_secs();
    let speed_factor = 0.8; 

    for (entity, mut transform, proj) in &mut query {
        let elapsed = now - proj.start_time;
        let progress = (elapsed * speed_factor).sin();

        if elapsed * speed_factor > std::f32::consts::PI {
            commands.entity(entity).despawn();
            continue;
        }        

        if progress < 0.0 { 
                continue; 
            }

        let current_dist: f32 = progress * proj.custom_distance;

        let spiral_effect = elapsed * 1.5; 
        let final_angle = proj.angle + spiral_effect;
        let offset_x = final_angle.cos() * current_dist;
        let offset_y = final_angle.sin() * current_dist;

        transform.translation.x = proj.start_pos.x + offset_x;
        transform.translation.y = proj.start_pos.y + offset_y;
    }
}

fn update_diagonal_sprites(
    time:  Res<Time>,
    mut despawner_query: Query<(&mut Sprite, &mut DiagonalMovementDespawner)>,
) {
    for (mut sprite, mut projectile) in &mut despawner_query {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            projectile.animation_timer.tick(time.delta());

            if projectile.animation_timer.just_finished() {
                if atlas.index >= 4 {
                    atlas.index = 0
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}