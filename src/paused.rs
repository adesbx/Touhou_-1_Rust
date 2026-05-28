use bevy::ecs::relationship::RelationshipSourceCollection;
use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub struct PausedPlugin;

impl Plugin for PausedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            switch_pause,
        ));

        app.add_systems(OnEnter(GameState::Paused), display_pause_menu);
        app.add_systems(OnExit(GameState::Paused), remove_pause_menu);
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
) {
    commands.spawn((
        PauseMenu,
        Sprite {
            color: Color::srgb(0.2, 0.2, 0.8),
            custom_size: Some(Vec2::new(400.0, 400.0)),
            ..default()
        },
        Transform::from_xyz(0.0 ,0.0, 900.0),
    ));
}

fn remove_pause_menu(
    mut commands: Commands, 
    pause_menu: Query<Entity, With<PauseMenu>>,
) {
    for entity in pause_menu{
        commands.entity(entity).despawn();
    }
}
