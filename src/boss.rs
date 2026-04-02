use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::constants::*;

pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_boss, show_health_bar));
    }
}

pub fn move_boss(
    time: Res<Time>,
    mut boss_query: Query<&mut Transform, With<Boss>>,
) {
    for mut transform in &mut boss_query {
        let target_y = 150.0;
        if transform.translation.y > target_y {
            transform.translation.y -= 100.0 * time.delta_secs();
            
            if transform.translation.y < target_y {
                transform.translation.y = target_y;
            }
        }
    }
}

pub fn show_health_bar(
    mut commands: Commands,
    boss_query: Query<Entity, With<Boss>>,
    bar_query: Query<Entity, With<BossHealthBar>>, 
) {
    if !boss_query.is_empty() && bar_query.is_empty() {
        commands.spawn((
            Sprite::from_color(
                Color::srgb(0.8, 0.1, 0.1),
                Vec2::new(300.0, 20.0),
            ),
            Transform::from_xyz(0.0, -GAME_HEIGHT / 2.0 + 30.0, 20.0),
            BossHealthBar,
        ));
    }
}