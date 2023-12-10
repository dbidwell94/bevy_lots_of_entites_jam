mod components;
mod systems;

use bevy::prelude::*;

use crate::GameState;

pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Main), systems::initial_pawn_spawn);
    }
}
