use bevy::ecs::relationship::RelationshipSourceCollection;
use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use bevy::asset::{io::Reader, AssetLoader, LoadContext};

pub struct DiscussionPlugin;

impl Plugin for DiscussionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            spawn_dialog_box,
            start_discussion,
            spawn_heroes,
        ).run_if(in_state(GameState::Discussion)));
    }
}

pub fn start_discussion(
    mut commands: Commands,
    dialogue_handle: Res<DialogueHandle>, 
    dialogue_assets: Res<Assets<Dialogue>>,
    mut text_query: Query<&mut Text, With<DialogueText>>,
    mut current_index: Local<Option<usize>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    dialog_query: Single<Entity, With<DialogueBox>>,
    mut witch_hero_query: Single<(Entity, &mut Sprite), (With<WitchHero>, Without<AngelHero>)>,
    mut angel_hero_query: Single<(Entity, &mut Sprite), (With<AngelHero>, Without<WitchHero>)>,
) {
    let Some(dialogue_data) = dialogue_assets.get(&dialogue_handle.0) else {
        println!("Asset Dialogue non trouvé ou non chargé !");
        return; 
    };

    let (witch_entity, witch_bg) = &mut *witch_hero_query; 
    let (angel_entity, angel_bg) = &mut *angel_hero_query; 

    if !witch_entity.is_empty() && !angel_entity.is_empty() {
        if keyboard.just_pressed(KeyCode::KeyK) {
            match *current_index {
                None => {
                    *current_index = Some(0);
                },
                Some(index) => {
                    *current_index = Some(index+1);
                    if index+1 >= dialogue_data.dialogues.len() {
                        *current_index = Some(0);
                        commands.set_state(GameState::Running);
                        commands.entity(dialog_query.entity()).despawn();
                        commands.entity(*witch_entity).despawn();
                        commands.entity(*angel_entity).despawn();
                    }

                }
            }

            if let Some(idx) = *current_index {
                let line: &DialogueLine = &dialogue_data.dialogues[idx];
                println!("{}: {}", line.speaker, line.text);

                let speak_color = Color::WHITE;
                let silent_color = Color::srgba(0.3, 0.3, 0.3, 1.0);

                if line.speaker == "Witch" {
                    witch_bg.color = speak_color;  
                    angel_bg.color = silent_color;  
                } else if line.speaker == "Angel" {
                    witch_bg.color = silent_color;  
                    angel_bg.color = speak_color;
                }

                if let Ok(mut text) = text_query.single_mut() {
                    **text = format!("{}: {}", line.speaker, line.text);
                }
            }
        }
    }
}

pub fn spawn_dialog_box(
    mut commands: Commands,
    dialog_query: Query<Entity, With<DialogueBox>>,
    asset_serv: Res<AssetServer>,
) {
    if dialog_query.is_empty() {
        commands.spawn((
            Node {
                width: Val::Percent(40.0),
                height: Val::Px(100.0),
                position_type: PositionType::Absolute,
                left: Val::Percent(30.0),
                bottom: Val::Px(20.0),
                padding: UiRect::all(Val::Px(15.0)),
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)), 
            BorderColor {
                top: Color::srgba(1.0, 1.0, 1.0, 0.5),
                right: Color::srgba(1.0, 1.0, 1.0, 0.5),
                bottom: Color::srgba(1.0, 1.0, 1.0, 0.5),
                left: Color::srgba(1.0, 1.0, 1.0, 0.5)
            },
            DialogueBox,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(""), 
                TextFont {
                    font_size: 14.0,
                    font: asset_serv.load("PressStart2P-Regular.ttf"),
                    ..default()
                },
                TextColor(Color::WHITE),
                DialogueText,
            ));
        });
    }
}

pub fn spawn_heroes(
    mut commands: Commands,
    asset_serv: Res<AssetServer>,
    witch_hero_query: Query<Entity, With<WitchHero>>,
    angel_hero_query: Query<Entity, With<AngelHero>>,
) {
    if witch_hero_query.is_empty() && angel_hero_query.is_empty() {
        let half_width = GAME_WIDTH / 2.0;
        let half_height = GAME_HEIGHT / 2.0;

        let bottom_left = Vec2::new(-half_width + 75.0, -half_height + 80.0);
        let bottom_right = Vec2::new(half_width - 60.0, -half_height + 85.0);

        let witch_original_width = 2508.0;
        let witch_original_height = 3252.0;
        let witch_reduction = 0.92;

        commands.spawn((
            Sprite {
                image: asset_serv.load("hud/witch_hero.png"),
                custom_size: Some(Vec2::new(
                    witch_original_width * (1.0 - witch_reduction), 
                    witch_original_height * (1.0 - witch_reduction)
                )),
                ..default()
            },
            Transform::from_translation(bottom_left.extend(200.0)),
            WitchHero
        ));

        let angel_original_width = 2460.0;
        let angel_original_height = 3272.0;
        let angel_reduction = 0.92;

        commands.spawn((
            Sprite {
                image: asset_serv.load("hud/angel_hero.png"),
                custom_size: Some(Vec2::new(
                    angel_original_width * (1.0 - angel_reduction), 
                    angel_original_height * (1.0 - angel_reduction)
                )),
                ..default()
            },
            Transform::from_translation(bottom_right.extend(200.0)),
            AngelHero
        ));
    }
}

impl AssetLoader for DialogueLoader {
    type Asset = Dialogue;
    type Settings = ();
    type Error = Box<dyn std::error::Error + Send + Sync>;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let level = ron::de::from_bytes::<Dialogue>(&bytes)?;
        Ok(level)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}