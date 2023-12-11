use super::styles::*;
use crate::{GameResources, GameState};
use bevy::prelude::*;
use bevy_ui_dsl::{node, root, text};

pub struct GameStateUIPlugin;

impl Plugin for GameStateUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Main), game_state_ui);
    }
}

#[derive(Component)]
struct GameResourceCounter;

fn game_state_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_resources: Res<GameResources>,
) {
    let root_entity = root(
        root_full_screen(Some(JustifyContent::Start), Some(AlignItems::End)),
        &asset_server,
        &mut commands,
        |p| {
            node(top_right_anchor, p, |p| {});
        },
    );
}
