use bevy::app::App;
use bevy::prelude::*;

mod components;
mod constants;
mod player;
mod enemy;
mod projectile;
mod level;
mod ui;

use crate::components::*;
use crate::constants::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_asset::<LevelData>()
        .register_asset_loader(LevelDataLoader)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(projectile::ProjectilePlugin)
        .add_plugins(level::LevelPlugin)
        .add_plugins(ui::UiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_serv: Res<AssetServer>) {   
    let handle = asset_serv.load("enemies.ron");
    commands.insert_resource(LevelHandle(handle));
    commands.insert_resource(BombSpawner {
        spawn_timer: Timer::from_seconds(5.0, TimerMode::Once),
    });

    commands.spawn((
        Camera2d, 
        Projection::Orthographic(OrthographicProjection { 
            scaling_mode: bevy::camera::ScalingMode::AutoMin { 
                min_width: GAME_WIDTH, 
                min_height: GAME_HEIGHT
            },
            ..OrthographicProjection::default_2d()
        }),
    )); 

    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.1, 0.1, 0.9),
            Vec2::new(50.0, GAME_HEIGHT * 2.0),
        ),
        Transform::from_xyz(-GAME_WIDTH / 2.0 - 50.0 / 2.0, 0.0, -10.0),
    ));

    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.1, 0.1, 0.9),
            Vec2::new(200.0, GAME_HEIGHT * 2.0),
        ),
        Transform::from_xyz(GAME_WIDTH / 2.0 + 200.0 / 2.0, 0.0, -10.0),
    ));

    let texture = asset_serv.load("characters/character.png");

    commands.spawn((
        Sprite::from_image(texture),
        Transform::from_xyz(0., 0., 0.),
        Player { 
            last_hit: 0.0, 
            shoot_timer: Timer::from_seconds(0.1, TimerMode::Repeating), 
            shoot_from_left: false,
            shoot_timer_fire: Timer::from_seconds(0.5, TimerMode::Repeating), 
            nbr_bombs: 0
        },
        Health { hp: PLAYER_HP},
        Damage { damage: PLAYER_DAMAGE}
    ));

    commands.spawn((
        Text::new("HP: 3"), 
        TextFont {
            font_size: 40.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(GAME_HEIGHT/2.0),
            left: Val::Px(GAME_HEIGHT/2.0 + 800.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.0, 0.0)), 
        PlayerHealthText, 
    ));

    commands.spawn((
        Text::new("Power: 10"), 
        TextFont {
            font_size: 40.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(GAME_HEIGHT/2.0 + 30.0),
            left: Val::Px(GAME_HEIGHT/2.0 + 800.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.0, 0.0)), 
        PlayerDamageText, 
    ));

    commands.spawn((
        Text::new("Bombs: 0"), 
        TextFont {
            font_size: 40.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(GAME_HEIGHT/2.0 + 60.0),
            left: Val::Px(GAME_HEIGHT/2.0 + 800.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.0, 0.0)), 
        PlayerBombsText, 
    ));
}
