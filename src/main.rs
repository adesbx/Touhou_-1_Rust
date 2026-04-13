use bevy::app::App;
use bevy::prelude::*;

mod components;
mod constants;
mod player;
mod enemy;
mod boss;
mod projectile;
mod level;
mod ui;
mod background;

use crate::components::*;
use crate::constants::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(LevelManager {
            current_phase: GamePhase::PreBoss,
            phase_timer: 0.0,
            next_index: 0,
        })
        .init_asset::<LevelData>()
        .register_asset_loader(LevelDataLoader)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(boss::BossPlugin)
        .add_plugins(projectile::ProjectilePlugin)
        .add_plugins(level::LevelPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(background::BackgroundPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, (play_main_theme, setup_assets))
        .run();
}

fn setup(
    mut commands: Commands, 
    asset_serv: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {   
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
            Sprite::from_image(asset_serv.load("hud/hud_bg.png")),
            Transform::from_xyz(80.0, 0.0, -100.0),
    ));

    let texture = asset_serv.load("characters/character.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 2, 3, None, None);
    let texture_atlas_layout = texture_atlas_layout.add(layout);

    commands.spawn((
        Sprite::from_atlas_image(texture, TextureAtlas { layout: texture_atlas_layout, index: 0}),
        Transform::from_xyz(0., 0., 0.),
        Player { 
            last_hit: 0.0, 
            shoot_timer: Timer::from_seconds(0.1, TimerMode::Repeating), 
            shoot_from_left: false,
            shoot_timer_fire: Timer::from_seconds(0.5, TimerMode::Repeating), 
            nbr_bombs: 0,
            animation_timer: Timer::from_seconds(0.2, TimerMode::Repeating), 
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
        TextColor(Color::srgb(1.0, 1.0, 1.0)), 
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
        TextColor(Color::srgb(1.0, 1.0, 1.0)), 
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
        TextColor(Color::srgb(1.0, 1.0, 1.0)), 
        PlayerBombsText, 
    ));
}


fn play_main_theme(
    asset_serv: Res<AssetServer>, 
    mut commands: Commands
) {
    commands.spawn((
        AudioPlayer::new(asset_serv.load("sounds/main_theme.ogg")),
        PlaybackSettings::LOOP,
    ));
}

fn setup_assets(mut commands: Commands, asset_serv: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        shoot_sound: asset_serv.load("sounds/player_shooting.ogg"),
        explosion_sound: asset_serv.load("sounds/explosion.ogg"),
    });
}
