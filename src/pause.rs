use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            switch_pause,
            button_system,
            button_system_action,
            sound_settigns_system
        ));

        app.add_systems(OnEnter(GameState::Paused), display_pause_menu);
        app.add_systems(OnExit(GameState::Paused), remove_pause_menu);
        app.add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup);
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
                DespawnOnExit(MenuState::Main),
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
                DespawnOnExit(MenuState::Main),
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
                DespawnOnExit(MenuState::Main),
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
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, action) in & interaction_query {
        if *interaction == Interaction::Pressed {
            match *action {
                MenuButtonAction::Reset => {
                    next_state.set(GameState::Reset);
                },
                MenuButtonAction::SettingsSound => {
                    menu_state.set(MenuState::SettingsSound);
                },
                MenuButtonAction::Resume => {
                    next_state.set(GameState::Running);
                },
                MenuButtonAction::Main => {
                    menu_state.set(MenuState::Main);
                },
            }
        }
    }
}

fn sound_settings_menu_setup(mut commands: Commands, volume: Res<VolumeButton>, pause_menu: Single<Entity, With<PauseMenu>>) {
    let button_node = Node {
        width: px(200),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = (
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(Color::WHITE),
    );

    let volume = *volume;
    let button_node_clone = button_node.clone();

    commands.entity(pause_menu.entity()).with_children(|parent| {
        parent.spawn((
            DespawnOnExit(MenuState::SettingsSound),
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            children![
                (
                    Node {
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Children::spawn((
                        Spawn((Text::new("Volume"), button_text_style.clone())),
                        SpawnWith(move |parent: &mut ChildSpawner| {
                            for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                                let mut entity = parent.spawn((
                                    Button,
                                    Node {
                                        width: px(30),
                                        height: px(65),
                                        ..button_node_clone.clone()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    VolumeButton(volume_setting),
                                ));

                                if volume == VolumeButton(volume_setting) {
                                    entity.insert(SelectedOption);
                                }
                            }
                        }),
                    )),
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Main,
                    children![(
                        Text::new("Back"),
                        button_text_style,
                    )]
                )
            ],
        ));
    });
}

fn sound_settigns_system(
    interaction_query: Query<
        (&Interaction, &VolumeButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut settings: ResMut<AudioSettings>,
) {
    for (interaction, action) in & interaction_query {
        if *interaction == Interaction::Pressed {
            settings.volume = action.0;
        }
    }
}
