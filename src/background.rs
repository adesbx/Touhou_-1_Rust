use bevy::prelude::*;
use crate::constants::*;
use crate::components::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_background)
           .add_systems(Update, scroll_background);
    }
}

fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("hud/cloud_bg.png");

    for i in 0..2 {
        commands.spawn((
            Sprite {
                image: texture.clone(),
                custom_size: Some(Vec2::new(GAME_WIDTH, GAME_HEIGHT)),
                ..default()
            },
            Transform::from_xyz(0.0, i as f32 * GAME_HEIGHT, -20.0), // on met la "prochaine image" au dessus de l'actuelle
            Background,
        ));
    }
}

fn scroll_background(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Background>>,
) {
    let scroll_speed = 50.0;

    for mut transform in &mut query {
        transform.translation.y -= scroll_speed * time.delta_secs();

        if transform.translation.y <= -GAME_HEIGHT { // quand l'image est en bas de l'écran
            transform.translation.y += GAME_HEIGHT * 2.0; // on la fait remonter tout en haut
        }
    }
}