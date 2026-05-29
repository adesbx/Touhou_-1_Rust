use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub struct PausedPlugin;

impl Plugin for PausedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            switch_pause,
            button_system,
            button_system_action
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
    asset_serv: Res<AssetServer>,
) {
    commands.spawn((
        PauseMenu,
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: percent(100),
            height: percent(100),
            ..default()
        },
        GlobalZIndex(900),
        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.0)),
    )).with_children(|parent|{
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(px(4)),
                width: percent(40),
                height: percent(60),
                ..default()
            },
            BackgroundColor(Color::srgb(0.07, 0.07, 0.3)),
            BorderColor::all(Color::WHITE),
            BorderRadius::new(Val::Px(25.0), Val::Px(25.0), Val::Px(25.0), Val::Px(25.0)),
        )).with_children(|menu| {
            let button_node = Node {
                width: px(300),
                height: px(65),
                margin: UiRect::all(px(20)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            };

            menu.spawn((
                Button,
                button_node.clone(),
                MenuButtonAction::Reset,
                BackgroundColor(Color::srgb(0.2, 0.4, 0.6)),
            )).with_children(|button|{
                button.spawn((
                    Text::new("Reset"),
                    TextFont {
                        font: asset_serv.load("PressStart2P-Regular.ttf"),
                        ..default()
                    },
                ));
            });

            menu.spawn((
                Button,
                button_node.clone(),
                MenuButtonAction::SettingsSound,
                BackgroundColor(Color::srgb(0.2, 0.4, 0.6)),
            )).with_children(|button|{
                button.spawn((
                    Text::new("Sound"),
                    TextFont {
                        font: asset_serv.load("PressStart2P-Regular.ttf"),
                        ..default()
                    },
                ));
            });

            menu.spawn((
                Button,
                button_node.clone(),
                MenuButtonAction::Resume,
                BackgroundColor(Color::srgb(0.2, 0.4, 0.6)),
            )).with_children(|button: &mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>|{
                button.spawn((
                    Text::new("Resume"),
                    TextFont {
                        font: asset_serv.load("PressStart2P-Regular.ttf"),
                        ..default()
                    },
                ));
            });
        });
    });
}

fn remove_pause_menu(
    mut commands: Commands, 
    pause_menu: Query<Entity, With<PauseMenu>>,
) {
    for entity in pause_menu{
        commands.entity(entity).despawn();
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn button_system_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, action) in & interaction_query {
        if *interaction == Interaction::Pressed {
            match *action {
                MenuButtonAction::Reset => {
                    next_state.set(GameState::Reset);
                },
                MenuButtonAction::SettingsSound => {
                    println!("Need to implement")
                },
                MenuButtonAction::Resume => {
                    next_state.set(GameState::Running);
                },
            }
        }
    }
}



