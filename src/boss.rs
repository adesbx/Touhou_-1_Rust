use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::constants::*;

pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_boss, show_health_bar, update_health_bar));
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
            Text::new("Boss HP"), 
            Sprite::from_color(
                Color::srgb(0.8, 0.1, 0.1),
                Vec2::new(300.0, 20.0),
            ),
            Transform::from_xyz(0.0, -GAME_HEIGHT / 2.0 + 30.0, 20.0),
            BossHealthBar,
        ));
    }
}

pub fn update_health_bar(
    mut boss_query: Query<&mut Health, With<Boss>>,
    bar_query: Query<(&mut Sprite, &mut Transform), With<BossHealthBar>>, 
) {
    let max_hp = BOSS_HP;
    let initial_width = 300.0;
    let initial_height: f32 = 20.0;

    if !boss_query.is_empty() && !bar_query.is_empty() {
            for (mut sprite, mut transform) in bar_query {
                    for health in &mut boss_query {
                        let current_width = health.hp / max_hp * initial_width;
                        sprite.custom_size = Some(Vec2::new(current_width, initial_height));

                        let offset_x = (initial_width - current_width) / 2.0;
            
                        transform.translation.x = offset_x;
                    }
            }
    }
}