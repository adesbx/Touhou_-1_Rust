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
            sound_settigns_system,
            reset_unselected_buttons
        ));

        app.add_systems(OnEnter(GameState::Paused), display_pause_menu);
        app.add_systems(OnExit(GameState::Paused), remove_pause_menu);
        app.add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup);
        app.add_systems(OnEnter(MenuState::Main), main_menu_setup);
    }
}

fn switch_pause(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if state.get() != &GameState::Paused {
            commands.set_state(GameState::Paused);
        } else {
            menu_state.set(MenuState::Nothing);
            commands.set_state(GameState::Running);
        }
    }
}

fn display_pause_menu(
    mut commands: Commands,
    mut menu_state: ResMut<NextState<MenuState>>,
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
            PauseMenuChildren,
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
        ));
    });
    menu_state.set(MenuState::Main);
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
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
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
                    menu_state.set(MenuState::Nothing);
                },
                MenuButtonAction::Main => {
                    menu_state.set(MenuState::Main);
                },
            }
        }
    }
}

fn main_menu_setup(
    mut commands: Commands,
    pause_menu: Single<Entity, With<PauseMenuChildren>>,
    asset_serv: Res<AssetServer>,
) {
    let button_node = Node {
        width: px(300),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let font = asset_serv.load("PressStart2P-Regular.ttf");

    commands
        .entity(pause_menu.entity())
        .with_children(|menu| {

            menu.spawn((
                DespawnOnExit(MenuState::Main),
                Button,
                button_node.clone(),
                MenuButtonAction::Reset,
                BackgroundColor(Color::srgb(0.2, 0.4, 0.6)),
            ))
            .with_children(|button| {
                button.spawn((
                    Text::new("Reset"),
                    TextFont {
                        font: font.clone(),
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
            ))
            .with_children(|button| {
                button.spawn((
                    Text::new("Sound"),
                    TextFont {
                        font: font.clone(),
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
            ))
            .with_children(|button| {
                button.spawn((
                    Text::new("Resume"),
                    TextFont {
                        font: font.clone(),
                        ..default()
                    },
                ));
            });
        });
}

fn sound_settings_menu_setup(
    mut commands: Commands, 
    pause_menu: Single<Entity, With<PauseMenuChildren>>,
    asset_serv: Res<AssetServer>,
) {
    let button_node = Node {
        width: px(100),
        height: px(45),
        margin: UiRect::all(px(5)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = (
        TextFont {
            font_size: 23.0,
            font: asset_serv.load("PressStart2P-Regular.ttf"),
            ..default()
        },
        TextColor(Color::WHITE),
    );

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
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: px(10),
                        ..default()
                    },
                    Children::spawn((
                        Spawn((
                            Text::new("Volume"),
                            button_text_style.clone(),
                        )),
                        SpawnWith(move |parent: &mut ChildSpawner| {
                            parent.spawn((
                                Node {
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                Children::spawn((
                                    SpawnWith(move |parent: &mut ChildSpawner| {
                                        for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                                            parent.spawn((
                                                Button,
                                                Node {
                                                    width: px(30),
                                                    height: px(65),
                                                    ..button_node_clone.clone()
                                                },
                                                BackgroundColor(NORMAL_BUTTON),
                                                VolumeButton(volume_setting),
                                            ));
                                        }
                                    }),
                                )),
                            ));
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
    mut commands: Commands, 
    interaction_query: Query<
        (Entity, &Interaction, &VolumeButton),
        (Changed<Interaction>, With<Button>),
    >,
    selected: Query<Entity, With<SelectedOption>>,
    mut settings: ResMut<AudioSettings>,
) {
    for (entity, interaction, action) in & interaction_query {
        if *interaction == Interaction::Pressed {
            settings.volume = action.0; 

            for old in &selected {
                commands.entity(old).remove::<SelectedOption>();
            }

            commands.entity(entity).insert(SelectedOption);
        }
    }
}

fn reset_unselected_buttons(
    mut query: Query<(&Interaction, &mut BackgroundColor, Option<&SelectedOption>), With<Button>>,
) {
    for (interaction, mut color, selected) in &mut query {
        if selected.is_none() && *interaction == Interaction::None {
            *color = NORMAL_BUTTON.into();
        }
    }
}