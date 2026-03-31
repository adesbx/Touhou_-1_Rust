use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component)]
pub struct Player {
    pub last_hit: f32,
    pub shoot_timer: Timer,
    pub shoot_timer_fire: Timer,
    pub shoot_from_left: bool,
    pub nbr_bombs: i32
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
}

#[derive(Component)]
pub struct EnemyProjectile {
    pub velocity: Vec2,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum MovePattern {
    Straight,
    ZigZag(f32),
    Spiral(f32),
    Arc(f32),
    SineWave,
}

#[derive(Component)]
pub struct EnemyMovement {
    pub spawn_time: f32,
    pub direction: f32, // 1.0 pour la droite vers la gauche, -1.0 pour l'inverse
    pub pattern: MovePattern,
}

#[derive(Component)]
pub struct Enemy {
    pub variety: char,
}

#[derive(Component, Debug, Deserialize)]
pub struct Health {
    pub hp: f32
}

#[derive(Component)]
pub struct Damage {
    pub damage: f32
}

#[derive(Deserialize, Debug)]
pub struct  EnemyWave {
    pub spawn_time: f32,
    pub pos: Vec2,
    pub direction: f32,
    pub hp: Health,
    pub variety: char,
    pub pattern: MovePattern
}


#[derive(Deserialize, Asset, TypePath, Debug)]
pub struct LevelData {
    pub waves: Vec<EnemyWave>
}

#[derive(Resource)]
pub struct LevelHandle(
    pub Handle<LevelData>
);

#[derive(Default)]
pub struct LevelDataLoader;