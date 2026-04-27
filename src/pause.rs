use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use bevy::asset::{io::Reader, AssetLoader, LoadContext};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            start_discution,
        ).run_if(in_state(GameState::Running)));
    }
}

pub fn start_discution(
    mut commands: Commands,
    asset_serv: Res<AssetServer>,
    dialogue_handle: Res<DialogueHandle>, 
    dialogue_assets: Res<Assets<Dialogue>>,
    mut text_query: Query<&mut Text, With<DialogueText>>,
) {
    let half_width = GAME_WIDTH / 2.0;
    let half_height = GAME_HEIGHT / 2.0;

    let bottom_left = Vec2::new(-half_width + 100.0, -half_height + 100.0);
    let bottom_right = Vec2::new(half_width - 100.0, -half_height + 100.0);

    let witch_original_width = 2508.0;
    let witch_original_height = 3252.0;
    let witch_reduction = 0.92;

    commands.spawn((
        Sprite {
            image: asset_serv.load("witch_v2.png"),
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
            image: asset_serv.load("TEST-COLOR.png"),
            custom_size: Some(Vec2::new(
                angel_original_width * (1.0 - angel_reduction), 
                angel_original_height * (1.0 - angel_reduction)
            )),
            ..default()
        },
        Transform::from_translation(bottom_right.extend(200.0)),
        AngelHero
    ));
    
    let Some(dialogue_data) = dialogue_assets.get(&dialogue_handle.0) else {
        println!("Asset Dialogue non trouvé ou non chargé !");
        return; 
    };

    let mut position_index = 0;
    // let line = &dialogue_data.dialogues[position_index];
    // if let Ok(mut text) = text_query.single_mut() {
    //     **text = format!("{}: {}", line.speaker, line.text);
    // }

    // commands.set_state(GameState::Running);
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