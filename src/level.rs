use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::constants::*;
use bevy::asset::{io::Reader, AssetLoader, LoadContext};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_from_level_data, spawn_bombs, move_bombs, check_collison_bombs, handle_despawn_timers));
    }
}

pub fn spawn_from_level_data(
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
            
            if wave.variety != 'b' {
                let mut text = "";
                if wave.variety == 'a'{
                    text = "enemies/angel.png";
                }
                else if wave.variety == 'c'{
                    text = "enemies/cherubin.png";
                }
                
                commands.spawn((
                    Sprite::from_image(asset_server.load(text)),
                    Transform::from_translation(wave.pos.extend(2.0)),
                    Enemy { variety: wave.variety},
                    Health { hp: wave.hp.hp },
                    EnemyMovement { spawn_time: current_time, direction: wave.direction, pattern: wave.pattern },
                ));
            } else {
                commands.spawn((
                    Sprite::from_image(asset_server.load("enemies/boss.png")),
                    Transform::from_translation(wave.pos.extend(2.0)),
                    Enemy { variety: wave.variety},
                    Health { hp: wave.hp.hp },
                    Boss { 
                        first_spawn: true, 
                        stop_normal_move: false,
                        phase: 1,
                        next_movement_timer: Timer::from_seconds(2.0, TimerMode::Repeating), 
                        next_position: Vec3 { x: wave.pos.x, y: wave.pos.y, z: 2.0},
                        basic_shoot_timer: Timer::from_seconds(1.0, TimerMode::Repeating), 
                        rain_shoot_timer: Timer::from_seconds(2.5, TimerMode::Repeating), 
                        current_attack: 1,
                        attack_switch_timer: Timer::from_seconds(5.0, TimerMode::Repeating), 
                    }
                ));
            }

            *next_index += 1;
        }
    }
}

pub fn spawn_bombs(
    time: Res<Time>,
    mut commands: Commands,
    asset_serv: Res<AssetServer>,
    mut spawner: ResMut<BombSpawner>,
){
    spawner.spawn_timer.tick(time.delta());

    if spawner.spawn_timer.is_finished() {
        let mut rng = rand::thread_rng();

        let texture: Handle<Image> = asset_serv.load("items/bomb_angel.png");
        let random_x = rng.gen_range(-GAME_WIDTH / 2.0 .. GAME_WIDTH / 2.0);
        let spawn_pos = Vec3::new(random_x, GAME_HEIGHT / 2.0 + 50.0, 5.0);

        commands.spawn((
            Sprite::from_image(texture),
            Transform::from_translation(spawn_pos),
            Bomb
        ));

        let next_wait = rng.gen_range(25.0..50.0);
        spawner.spawn_timer.set_duration(std::time::Duration::from_secs_f32(next_wait));
        spawner.spawn_timer.reset();
    }
}

fn move_bombs(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Bomb>>,
) {
    for mut transform  in &mut query {
        transform.translation.y -= 40.0 * time.delta_secs();
    }
}

fn check_collison_bombs(
    mut commands: Commands,
    bomb_query: Query<(Entity, &Transform), With<Bomb>>,
    mut player_query: Single<(&Transform, &mut Player), With<Player>>,
) {
    let (transform, player) = &mut *player_query; // possiblement sale voir pour faire autrement

    for (power_entity, power_transform) in &bomb_query {
            let p1 = power_transform.translation.truncate(); // Vec3 -> Vec2
            let p2 = transform.translation.truncate();
            let distance: f32 = p1.distance(p2);
            if distance < (BOMB_SIZE + PLAYER_SIZE) / 2.0 {                
                player.nbr_bombs += 1;
                commands.entity(power_entity).despawn();

                break;
            }
    }
}

fn handle_despawn_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DespawnTimer)>,
) {
    for (entity, mut despawn_timer) in &mut query {
        despawn_timer.timer.tick(time.delta());
        
        if despawn_timer.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}


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