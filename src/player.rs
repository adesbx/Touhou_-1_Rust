use bevy::prelude::*;
use std::time::Duration;
use crate::components::*;
use crate::constants::*;
use bevy::audio::Volume;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            move_player, 
            confine_player_movement, 
            move_power_up, 
            check_collison_power_up, 
            use_bombs,
            change_color_on_hit, 
            update_player_sprites, 
            update_explosion_sprite,
            check_if_dead
        ).run_if(in_state(GameState::Running).or(in_state(GameState::Paused))));
    }
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

    if keyboard.pressed(KeyCode::ShiftLeft) {
        player_transform.translation.x += direction.x * time.delta_secs() * PLAYER_SPEED * 0.5;
        player_transform.translation.y += direction.y * time.delta_secs() * PLAYER_SPEED * 0.5;
    } else {
        player_transform.translation.x += direction.x * time.delta_secs() * PLAYER_SPEED;
        player_transform.translation.y += direction.y * time.delta_secs() * PLAYER_SPEED;
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
    mut player_query: Single<(&Transform, &mut Damage, &mut Player), With<Player>>,
) {
    let (transform, damage_player, player) = &mut *player_query; // possiblement sale voir pour faire autrement

    for (power_entity, power_transform) in &power_up_query {
            let p1 = power_transform.translation.truncate(); // Vec3 -> Vec2
            let p2 = transform.translation.truncate();
            let distance: f32 = p1.distance(p2);
            if distance < (POWER_UP_SIZE + PLAYER_SIZE) / 2.0 {                
                damage_player.damage += PLAYER_DAMAGE;
                commands.entity(power_entity).despawn();
                
                // ICI pour changer la vitesse de tir du joueur
                let new_delay = (0.1 - (damage_player.damage - 10.0) * 0.0005).max(0.01);
                player.shoot_timer.set_duration(Duration::from_secs_f32(new_delay));

                break;
            }
    }
}

fn use_bombs(
    mut commands: Commands,
    asset_serv: Res<AssetServer>,
    assets: Res<GameAssets>,
    mut player_query: Single<(&Transform, &mut Player), With<Player>>,
    mut enemy_query: Query<(&Transform, &mut Health), With<Enemy>>, 
    keyboard: Res<ButtonInput<KeyCode>>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {

    let (transform, player) = &mut *player_query; // possiblement sale voir pour faire autrement

    if keyboard.just_pressed(KeyCode::KeyL) && player.nbr_bombs > 0 {
        let player_pos = transform.translation.truncate();
        let bomb_radius = 180.0;
        let bomb_damage = 200.0;
        for (enemy_transform, mut enemy_health) in &mut enemy_query {
            let enemy_pos = enemy_transform.translation.truncate();
            if player_pos.distance(enemy_pos) < bomb_radius {
                enemy_health.hp -= bomb_damage;
            
            }
        }

        let texture: Handle<_> = asset_serv.load("projectiles/explosion_ring.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 2, 3, None, None);
        let texture_atlas_layout = texture_atlas_layout.add(layout);
        
        commands.spawn((
            Sprite {
                image: texture,
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                }),
                custom_size: Some(Vec2::new(128.0, 128.0)),
                ..default()
            },
            Transform::from_translation(transform.translation),
            DespawnTimer {
                timer: Timer::from_seconds(0.5, TimerMode::Once),
                animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            },
        ));

        commands.spawn((
            AudioPlayer::new(assets.explosion_sound.clone()),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: Volume::Decibels(-5.0),
                ..default()
            },
        ));

        player.nbr_bombs -= 1;
    }

}

fn change_color_on_hit(
    time:  Res<Time>,
    mut player_query: Single<(&mut Player, &mut Sprite), With<Player>>,
) {
    let (player, sprite) = &mut *player_query; // possiblement sale voir pour faire autrement
    if time.elapsed_secs() - player.last_hit < INVINCIBILITY_TIME && player.last_hit != 0.0 {
        sprite.color = Color::srgba(0.9, 0.0, 0.0, 0.9);
    } else {
        sprite.color = Color::WHITE;
    }
}

fn update_player_sprites(
    time:  Res<Time>,
    mut player: Single<(&mut Sprite, &mut Player), With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let (sprite, player) = &mut *player;

    if let Some(atlas) = sprite.texture_atlas.as_mut() {


        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::KeyA) {
            if keyboard.pressed(KeyCode::KeyD) {
                atlas.index = 3;
            }

            if keyboard.pressed(KeyCode::KeyA) {
                atlas.index = 4;
            }
        } else {
            player.animation_timer.tick(time.delta());

            if player.animation_timer.just_finished() {
                if atlas.index >=2 {
                    atlas.index = 0
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}

fn update_explosion_sprite(
    time:  Res<Time>,
    explosion: Single<(&mut Sprite, &mut DespawnTimer), With<DespawnTimer>>,
) {

    let (mut sprite, mut timer) = explosion.into_inner();

    if let Some(atlas) = sprite.texture_atlas.as_mut() {
        timer.animation_timer.tick(time.delta());

        if timer.animation_timer.just_finished() {
            if atlas.index < 4 {
                atlas.index += 1;
            }
        }
    }
}

fn check_if_dead(
    mut next_state: ResMut<NextState<GameState>>,
    player_query: Query<Entity, With<Player>>,
) {
    if player_query.is_empty() {
        next_state.set(GameState::Reset);
    }
}