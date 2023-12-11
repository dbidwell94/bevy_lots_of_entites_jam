use bevy::prelude::*;
use bevy_ui_dsl::{root, text};

use crate::GameState;

use super::styles::*;

pub struct FactoryStateUIPlugin;

impl Plugin for FactoryStateUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::FactoryPlacement), create_ui)
            .add_systems(OnExit(GameState::FactoryPlacement), destroy_ui);
    }
}

#[derive(Component)]
struct FactoryUI;

fn create_ui(mut commands: Commands, asset_server: Res<AssetServer>) {

    let parent = root(
        root_full_screen(Some(JustifyContent::Start), Some(AlignItems::Center)),
        &asset_server,
        &mut commands,
        |p| {
            text(
                "Use left click to place your pawn factory. Pawns will spawn around the factory and proceed to collect resources from the nearest ore deposit.",
                c_pixel_text,
                text_style(None),
                p,
            );
        },
    );

    commands
        .entity(parent)
        .insert((FactoryUI, Name::new("FactorySpawningUI")));
}

fn destroy_ui(mut commands: Commands, query: Query<Entity, With<FactoryUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
