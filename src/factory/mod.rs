pub mod components;
mod systems;

use crate::GameState;
use bevy::prelude::*;

pub use components::*;

pub struct FactoryPlugin;

impl Plugin for FactoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::WorldSpawn),
            systems::initial_spawn_factory,
        )
        .add_systems(
            Update,
            (
                systems::clamp_factory_to_cursor_position
                    .run_if(in_state(GameState::FactoryPlacement)),
                systems::place_factory
                    .after(systems::clamp_factory_to_cursor_position)
                    .run_if(in_state(GameState::FactoryPlacement)),
            ),
        );
    }
}
