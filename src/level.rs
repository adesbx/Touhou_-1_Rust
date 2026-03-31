use bevy::prelude::*;
use crate::components::*;
use bevy::asset::{io::Reader, AssetLoader, LoadContext};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_from_level_data);
    }
}

pub fn spawn_from_level_data(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_assets: Res<Assets<LevelData>>,
    level_handle: Res<LevelHandle>, 
    mut next_index: Local<usize>,
) {    
    if let Some(level) = level_assets.get(&level_handle.0) {
        let current_time = time.elapsed_secs();

        while *next_index < level.waves.len() && current_time >= level.waves[*next_index].spawn_time {
            let wave = &level.waves[*next_index];
            
            let mut text = "";
            if wave.variety == 'a'{
                text = "enemies/angel.png";
            }
            else if wave.variety == 'c'{
                text = "enemies/cherubien.png";
            }
            
            commands.spawn((
                Sprite::from_image(asset_server.load(text)),
                Transform::from_translation(wave.pos.extend(2.0)),
                Enemy { variety: wave.variety},
                Health { hp: wave.hp.hp },
                EnemyMovement { spawn_time: current_time, direction: wave.direction, pattern: wave.pattern },
            ));

            *next_index += 1;
        }
    }
}

impl AssetLoader for LevelDataLoader {
    type Asset = LevelData;
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
        let level = ron::de::from_bytes::<LevelData>(&bytes)?;
        Ok(level)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}