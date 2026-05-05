use bevy::app::App;
use bevy::ecs::relationship::RelationshipSourceCollection;
use bevy::prelude::*;
use bevy::audio::Volume;

mod components;
mod constants;
mod player;
mod enemy;
mod boss;
mod projectile;
mod level;
mod ui;
mod background;
mod pause;

use crate::components::*;
use crate::constants::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<GameState>()
        .init_asset::<LevelData>()
        .init_asset::<Dialogue>()
        .insert_resource(LevelManager {
            current_phase: GamePhase::PreBoss,
            phase_timer: 0.0,
            next_index: 0,
            power_up_timer: Timer::from_seconds(2.0, TimerMode::Repeating), 
        })
        .register_asset_loader(LevelDataLoader)
        .register_asset_loader(DialogueLoader)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(boss::BossPlugin)
        .add_plugins(projectile::ProjectilePlugin)
        .add_plugins(level::LevelPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(background::BackgroundPlugin)
        .add_plugins(pause::PausePlugin)
        .add_systems(Startup, (setup, setup_assets))
        .add_systems(Update, play_music_theme)
        .run();
}

fn setup(
    mut commands: Commands, 
    asset_serv: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {   
    let handle = asset_serv.load("enemies.ron");
    commands.insert_resource(LevelHandle(handle));
    let handle = asset_serv.load("dialogue.ron");
    commands.insert_resource(DialogueHandle(handle));
    commands.insert_resource(BombSpawner {
        spawn_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
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
            Sprite {
                image: asset_serv.load("hud/hud_bg.png"),
                custom_size: Some(Vec2::new(900.0, 448.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, -100.0),
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
        Health { hp: PLAYER_HP, ..default()},
        Damage { damage: PLAYER_DAMAGE}
    ));

    commands.spawn((
        Text::new(format!("HP:{:.0}", PLAYER_HP)), 
        TextFont {
            font_size: 32.0,
            font: asset_serv.load("PressStart2P-Regular.ttf"),
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(GAME_HEIGHT/2.0),
            left: Val::Px(GAME_HEIGHT/2.0 + 750.0),
            ..default()
        },
        PlayerHealthText, 
    ));

    commands.spawn((
        Text::new("Power:10"), 
        TextFont {
            font_size: 32.0,
            font: asset_serv.load("PressStart2P-Regular.ttf"),
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(GAME_HEIGHT/2.0 + 30.0),
            left: Val::Px(GAME_HEIGHT/2.0 + 750.0),
            ..default()
        },
        PlayerDamageText, 
    ));

    commands.spawn((
        Text::new("Bombs:0"), 
        TextFont {
            font_size: 32.0,
            font: asset_serv.load("PressStart2P-Regular.ttf"),
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(GAME_HEIGHT/2.0 + 60.0),
            left: Val::Px(GAME_HEIGHT/2.0 + 750.0),
            ..default()
        },
        PlayerBombsText, 
    ));

    commands.spawn((
        Text::new("Touhou-1"), 
        TextFont {
            font_size: 40.0,
            font: asset_serv.load("PressStart2P-Regular.ttf"),
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(GAME_HEIGHT/2.0 + 30.0),
            left: Val::Px(0.0),
            ..default()
        },
    ));
}


fn play_music_theme(
    asset_serv: Res<AssetServer>, 
    mut commands: Commands,
    manager: Res<LevelManager>,
    music_query: Query<Entity, With<MusicPlayed>>,
    mut current_music: Local<String>,
) {

    if !music_query.is_empty() {
        if manager.current_phase != GamePhase::BossFight && manager.current_phase != GamePhase::PreBoss {
            return;
        }

        let sound_path = if manager.current_phase == GamePhase::BossFight {
            "sounds/boss_theme.ogg"
        } else {
            "sounds/main_theme.ogg"
        };

        let volume = if manager.current_phase == GamePhase::BossFight {
            Volume::Decibels(-3.0)
        } else {
            Volume::Decibels(-7.0)
        };

        if *current_music == sound_path {
            return;
        }

        for entity in &music_query {
            commands.entity(entity).despawn();
        }


        commands.spawn((
            AudioPlayer::new(asset_serv.load(sound_path)),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: volume,
                ..default()
            },
            MusicPlayed
        ));
        *current_music =  sound_path.to_string();
    } else {
        commands.spawn((
            AudioPlayer::new(asset_serv.load( "sounds/main_theme.ogg")),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: Volume::Decibels(-7.0),
                ..default()
            },
            MusicPlayed
        ));
        *current_music =  "sounds/main_theme.ogg".to_string();
    }
}

fn setup_assets(mut commands: Commands, asset_serv: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        shoot_sound: asset_serv.load("sounds/player_shooting.ogg"),
        explosion_sound: asset_serv.load("sounds/explosion.ogg"),
        cross_electricity: asset_serv.load("sounds/cross_electricity.ogg"),
        vortex_explosion: asset_serv.load("sounds/vortex_explosion.ogg"),
        shoot_fire_sound: asset_serv.load("sounds/fire_projectile.ogg"),
    });
}
