use bevy::ecs::relationship::RelationshipSourceCollection;
use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub struct PausedPlugin;

impl Plugin for PausedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            switch_pause,
            display_pause_menu
        ));
    }
}

fn switch_pause(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if state.get() != &GameState::Paused {
            commands.set_state(GameState::Paused);
        } else {
            commands.set_state(GameState::Running);
        }
    }
}

fn display_pause_menu(
    mut commands: Commands,
    state: Res<State<GameState>>,
    pause_menu: Single<Entity, With<PauseMenu>>,
) {
    if state.get() == &GameState::Running {
        commands.spawn((
        PauseMenu,
        Sprite {
            color: Color::srgb(0.2, 0.2, 0.2),
            custom_size: Some(Vec2::new(400.0, 250.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10000000000000.0),
        ));
    } else {
        commands.entity(pause_menu.entity()).despawn();
    }
}
