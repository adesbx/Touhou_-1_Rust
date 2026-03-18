use bevy::app::App;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const PLAYER_SPEED: f32 = 200.0;
const PLAYER_SIZE: f32 = 24.0; // taille du sprite du joueur
const ZOOM_FACTOR: f32 = 0.5; // zoom caméra

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .add_systems(Update, confine_player_movement)
        .run();
}

fn setup(mut commands: Commands, asset_serv: Res<AssetServer>) {
    commands.spawn((
        Camera2d, 
        Projection::Orthographic(OrthographicProjection { 
            scale: ZOOM_FACTOR,
            ..OrthographicProjection::default_2d()
        }),
    )); 

    let texture = asset_serv.load("characters/square.png");

    commands.spawn((
        Sprite::from_image(texture),
        Transform::from_xyz(0., 0., 0.),
        Player,
    ));
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

    player_transform.translation.x += direction.x * time.delta_secs() * PLAYER_SPEED;
    player_transform.translation.y += direction.y * time.delta_secs() * PLAYER_SPEED;
}

fn confine_player_movement(
    mut player_transform: Single<&mut Transform, With<Player>>, 
    window: Single<&Window, With<PrimaryWindow>>
) {

    let half_player_size: f32 = (PLAYER_SIZE / 2.0) * ZOOM_FACTOR;
    let half_width: f32 = (window.width() / 2.0) * ZOOM_FACTOR;
    let half_height: f32 = (window.height() / 2.0) * ZOOM_FACTOR;

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
 
#[derive(Component)]
struct Player;