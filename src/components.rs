use bevy::prelude::*;
use serde::Deserialize;
use bevy::time::Stopwatch;

#[derive(Component, Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Health {
    pub hp: f32,
    #[serde(skip)]
    pub is_dying : bool,
    #[serde(skip)]
    pub dying_timer: Timer,
}

#[derive(Component)]
pub struct Damage {
    pub damage: f32
}

#[derive(Component)]
pub struct PowerUp;

#[derive(Component)]
pub struct Bomb;

#[derive(Resource)]
pub struct BombSpawner{
    pub spawn_timer: Timer,
}

#[derive(Component)]
pub struct DespawnTimer {
    pub timer: Timer,
    pub animation_timer: Timer,
}

#[derive(Component)]
pub struct Player {
    pub last_hit: f32,
    pub shoot_timer: Timer,
    pub shoot_timer_fire: Timer,
    pub shoot_from_left: bool,
    pub bomb_timer: Timer,
    pub nbr_bombs: i32,
    pub animation_timer: Timer,
}

#[derive(Component)]
pub struct PlayerHealthText;

#[derive(Component)]
pub struct PlayerDamageText;

#[derive(Component)]
pub struct PlayerBombsText;

#[derive(Component)]
pub struct Projectile {
    pub direction: Vec2,
    pub speed: f32,
    pub variety: char,
    pub spawn_time: f32,
    pub size: f32
}

#[derive(Component)]
pub struct BasicProjectileBoss {
    pub start_pos: Vec2,
    pub explosion_dist: f32,
}

#[derive(Component)]
pub struct VortexFragment {
    pub center: Vec2,      
    pub angle: f32,       
    pub radius: f32,     
    pub rotate_speed: f32,
    pub expand_speed: f32,
}

#[derive(Component)]
pub struct BoomerangProjectile {
    pub angle: f32,      
    pub start_pos: Vec3, 
    pub start_time: f32,
    pub custom_distance: f32,
}

#[derive(Component)]
pub struct EnemyProjectile {
    pub direction: Vec2,
    pub speed: f32,
    pub size: f32,
}

#[derive(Component)]
pub struct DiagonalMovementSpawner {
    pub x: f32,
    pub y: f32,
    pub spawn_time: f32,
}

#[derive(Component)]
pub struct DiagonalMovementDespawner {
    pub spawn_time: f32,
    pub animation_timer: Timer
}

#[derive(Component)]
pub struct Enemy {
    pub variety: char,
    pub animation_timer: Timer,
    pub shoot_timer: Timer
}

#[derive(Component)]
pub struct EnemyMovement {
    pub spawn_time: f32,
    pub direction: f32, // 1.0 pour la droite vers la gauche, -1.0 pour l'inverse
    pub pattern: MovePattern,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum MovePattern {
    Straight,
    ZigZag(f32),
    Spiral(f32),
    Arc(f32),
    SineWave,
    StraightPause(f32)
}

#[derive(Component)]
pub struct Boss {
    pub first_spawn: bool,
    pub stop_normal_move: bool,
    pub phase: u32,
    pub next_movement_timer: Timer,
    pub next_position: Vec3,
    pub basic_shoot_timer: Timer,
    pub rain_shoot_timer: Timer,
    pub current_attack: u32,
    pub attack_switch_timer: Timer,
    pub diagonal_attack_timer: Timer,
    pub boomerang_attack_timer: Timer,
    pub animation_timer: Timer,
}

#[derive(Component)]
pub struct BossHealthBar;

#[derive(Deserialize, Debug)]
pub struct EnemyWave {
    pub spawn_time: f32,
    pub pos: Vec2,
    pub direction: f32,
    pub hp: Health,
    pub variety: char,
    pub pattern: MovePattern
}


#[derive(Deserialize, Asset, TypePath, Debug)]
pub struct LevelData {
    pub pre_boss: Vec<EnemyWave>,
    pub boss: EnemyWave,
    pub post_boss: Vec<EnemyWave>,
}

#[derive(Resource)]
pub struct LevelManager {
    pub current_phase: GamePhase,
    pub phase_timer: f32,
    pub next_index: usize,
    pub power_up_timer: Timer,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
    Reset
}

#[derive(Deserialize, Asset, TypePath, Debug, PartialEq)]
pub enum GamePhase {
    PreBoss,
    FistBossEncounter,
    PostBoss,
    BossFight,
    Dialogue
}

#[derive(Deserialize, Asset, TypePath, Debug)]
pub struct Dialogue {
    pub dialogues: Vec<DialogueLine>,
}

#[derive(Deserialize, Debug)]
pub struct DialogueLine {
    pub speaker: String,
    pub text: String
}

#[derive(Resource)]
pub struct DialogueHandle(
    pub Handle<Dialogue>
);

#[derive(Default)]
pub struct DialogueLoader;

#[derive(Component)]
pub struct DialogueBox;

#[derive(Component)]
pub struct DialogueText;

#[derive(Component)]
pub struct WitchHero;

#[derive(Component)]
pub struct AngelHero;

#[derive(Resource)]
pub struct LevelHandle(
    pub Handle<LevelData>
);

#[derive(Default)]
pub struct LevelDataLoader;

#[derive(Component)]
pub struct Background;

#[derive(Resource, Default)]
pub struct GameClock {
    pub watch: Stopwatch,
}

#[derive(Component)]
pub struct HitBox;

#[derive(Resource)]
pub struct GameAssets {
    pub shoot_sound: Handle<AudioSource>,
    pub explosion_sound: Handle<AudioSource>,
    pub enemy_dying: Handle<AudioSource>,
    pub cross_electricity: Handle<AudioSource>,
    pub vortex_explosion: Handle<AudioSource>,
    pub shoot_fire_sound: Handle<AudioSource>,
}

#[derive(Component)]
pub struct MusicPlayed;

#[derive(Resource, Default)]
pub struct AudioSettings {
    pub is_muted: bool,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            hp: 100.0,
            is_dying: false,
            dying_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}