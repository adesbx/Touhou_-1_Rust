use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::constants::*;
use bevy::asset::{io::Reader, AssetLoader, LoadContext};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            spawn_from_level_data, 
            spawn_bombs, move_bombs, 
            check_collison_bombs, 
            spawn_power_up, 
            handle_despawn_timers
        ).run_if(in_state(GameState::Running)));
    }
}

pub fn spawn_from_level_data(
    time: Res<Time>,
    mut commands: Commands,
    asset_serv: Res<AssetServer>,
    level_assets: Res<Assets<LevelData>>,
    level_handle: Res<LevelHandle>, 
    mut manager: ResMut<LevelManager>, 
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    state: Res<State<GameState>>,
    enemy_query: Query<&Enemy, With<Enemy>>,
    projectile_query: Query<Entity, With<Projectile>>,
) {    
    if let Some(level) = level_assets.get(&level_handle.0) {
        manager.phase_timer += time.delta_secs();

        let current_waves = match manager.current_phase {
            GamePhase::PreBoss => &level.pre_boss,
            GamePhase::PostBoss => &level.post_boss,
            GamePhase::FistBossEncounter => {
                return;
            }
            GamePhase::BossFight => {
                return; //
            }
            GamePhase::Dialogue => {
                return; //
            }
        };

        while manager.next_index < current_waves.len() && manager.phase_timer >= current_waves[manager.next_index].spawn_time {
            let wave = &current_waves[manager.next_index];
            
            let mut texture_path = "enemies/angel.png";
            if wave.variety == 'c' { texture_path = "enemies/cherubin.png"; }

            let texture: Handle<_> = asset_serv.load(texture_path);
            let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 1, 2, None, None);
            let texture_atlas_layout = texture_atlas_layout.add(layout);
            
            commands.spawn((
                Sprite {
                    image: texture,
                    texture_atlas: Some(TextureAtlas { layout: texture_atlas_layout, index: 0}),
                    custom_size: if wave.variety == 'c' { Some(Vec2::new(CHERUB_SIZE, CHERUB_SIZE)) } else { Some(Vec2::new(ANGEL_SIZE, ANGEL_SIZE)) },
                    ..default()
                },
                Transform::from_translation(wave.pos.extend(2.0)),
                Enemy {
                    variety: wave.variety,
                    animation_timer: Timer::from_seconds(0.3, TimerMode::Repeating), 
                    // ICI pour changer la vitesse de tir d'un ennemi
                    shoot_timer: Timer::from_seconds(1.2, TimerMode::Repeating), 
                },
                Health { hp: wave.hp.hp, ..default()},
                EnemyMovement { 
                    spawn_time: manager.phase_timer,
                    direction: wave.direction, 
                    pattern: wave.pattern 
                },
            ));

            manager.next_index += 1;
        }

        if manager.current_phase == GamePhase::PreBoss && manager.next_index >= level.pre_boss.len() && enemy_query.iter().count() == 0 { // ajoute dimension temps
            manager.phase_timer = 0.0;
            manager.next_index = 0;
            manager.current_phase = GamePhase::FistBossEncounter;
            let wave = &level.boss;
            let texture = asset_serv.load("enemies/boss.png");
            let layout = TextureAtlasLayout::from_grid(UVec2::splat(28), 3, 4, None, None);
            let texture_atlas_layout = texture_atlas_layout.add(layout);

            commands.spawn((
                Sprite::from_atlas_image(texture, TextureAtlas { layout: texture_atlas_layout, index: 0}),
                Transform::from_translation(wave.pos.extend(2.0)),
                Enemy { 
                    variety: wave.variety,
                    animation_timer: Timer::from_seconds(0.3, TimerMode::Repeating), 
                    shoot_timer: Timer::from_seconds(0.0, TimerMode::Once),
                },
                Health { hp: wave.hp.hp, ..default()},
                Boss {
                    first_spawn: true,
                    stop_normal_move: false,
                    phase: 1,
                    // ICI pour changer la vitesse de tir, mouvement du boss
                    next_movement_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                    next_position: Vec3 { x: wave.pos.x, y: wave.pos.y, z: 2.0},
                    basic_shoot_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                    rain_shoot_timer: Timer::from_seconds(2.5, TimerMode::Repeating),
                    current_attack: 1,
                    attack_switch_timer: Timer::from_seconds(5.0, TimerMode::Repeating),
                    diagonal_attack_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
                    boomerang_attack_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                    animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                } 
            ));
        }

        if manager.current_phase == GamePhase::PostBoss && manager.next_index >= current_waves.len() && enemy_query.iter().count() == 0 {
            commands.set_state(GameState::Paused);
            manager.current_phase = GamePhase::Dialogue;

            for entity in projectile_query{
                commands.entity(entity).despawn();
            }
        }

        if manager.current_phase == GamePhase::Dialogue && state.get() == &GameState::Running && enemy_query.iter().count() == 0 {
            manager.phase_timer = 0.0;
            manager.next_index = 0;
            manager.current_phase = GamePhase::BossFight;
            let wave = &level.boss;
            let texture = asset_serv.load("enemies/boss.png");
            let layout = TextureAtlasLayout::from_grid(UVec2::splat(28), 3, 4, None, None);
            let texture_atlas_layout = texture_atlas_layout.add(layout);

            commands.spawn((
                Sprite::from_atlas_image(texture, TextureAtlas { layout: texture_atlas_layout, index: 8}),
                Transform::from_translation(wave.pos.extend(2.0)),
                Enemy { 
                    variety: wave.variety,
                    animation_timer: Timer::from_seconds(0.3, TimerMode::Repeating), 
                    shoot_timer: Timer::from_seconds(0.0, TimerMode::Once), 
                },
                Health { hp: wave.hp.hp, ..default()},
                Boss {
                    first_spawn: false,
                    stop_normal_move: false,
                    phase: 1,
                    // ICI pour changer la vitesse de tir, mouvement du boss
                    next_movement_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                    next_position: Vec3 { x: wave.pos.x, y: wave.pos.y, z: 2.0},
                    basic_shoot_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                    rain_shoot_timer: Timer::from_seconds(2.5, TimerMode::Repeating),
                    current_attack: 1,
                    attack_switch_timer: Timer::from_seconds(5.0, TimerMode::Repeating),
                    diagonal_attack_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
                    boomerang_attack_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                    animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                } 
            ));
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

        let next_wait = rng.gen_range(5.0..20.0);
        spawner.spawn_timer.set_duration(std::time::Duration::from_secs_f32(next_wait));
        spawner.spawn_timer.reset();
    }
}

pub fn spawn_power_up(
    time: Res<Time>,
    mut commands: Commands,
    asset_serv: Res<AssetServer>,
    mut manager: ResMut<LevelManager>, 
){


    manager.power_up_timer.tick(time.delta());

    if manager.power_up_timer.is_finished() {
        let mut rng = rand::thread_rng();

        let random_x = rng.gen_range(-GAME_WIDTH / 2.0 .. GAME_WIDTH / 2.0);
        let spawn_pos = Vec3::new(random_x, GAME_HEIGHT / 2.0 + 50.0, 5.0);

        commands.spawn((
            Sprite {
                image: asset_serv.load("items/power_up.png"),
                custom_size: Some(Vec2::new(POWER_UP_SIZE, POWER_UP_SIZE)),
                ..default()
            },
            Transform::from_translation(spawn_pos),
            PowerUp
        ));

        let next_wait = rng.gen_range(2.0..10.0);
        manager.power_up_timer.set_duration(std::time::Duration::from_secs_f32(next_wait));
        manager.power_up_timer.reset();
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