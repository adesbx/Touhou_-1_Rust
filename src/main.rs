use bevy::app::App;
use bevy::ecs::query;
use bevy::input::keyboard;
use bevy::prelude::*;

const  PLAYER_SPEED: f32 = 200.0;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

fn setup(mut commands: Commands, asset_serv: Res<AssetServer>) {
    commands.spawn((
        Camera2d, 
        Projection::Orthographic(OrthographicProjection { 
            scale: 0.1,
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

#[derive(Component)]
struct Player;