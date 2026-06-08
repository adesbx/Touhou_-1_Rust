use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_health_ui, update_damage_ui, update_bombs_ui));
        app.add_systems(OnEnter(GameState::EndGame), show_win_message);
    }
}

fn update_health_ui(
    player_query: Single<&Health, With<Player>>, 
    mut text_query: Single<&mut Text, With<PlayerHealthText>>,
) {
    text_query.0 = format!("HP:{:.0}", player_query.hp);
}

fn update_damage_ui(
    player_query: Single<&Damage, With<Player>>, 
    mut text_query: Single<&mut Text, With<PlayerDamageText>>,
) {
    text_query.0 = format!("Power:{:.0}", player_query.damage);
}

fn update_bombs_ui(
    player_query: Single<&Player, With<Player>>, 
    mut text_query: Single<&mut Text, With<PlayerBombsText>>,
) {
    text_query.0 = format!("Bombs:{:.0}", player_query.nbr_bombs);
}

fn show_win_message(
    mut commands: Commands, 
    asset_serv: Res<AssetServer>,
) {
    commands.spawn((
        Text::new("You win !"), 
        TextFont {
            font_size: 50.0,
            font: asset_serv.load("PressStart2P-Regular.ttf"),
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(GAME_HEIGHT/2.0),
            left: Val::Percent(35.0),
            ..default()
        },
        ZIndex(999)
    ));
}
