mod systems;

use bevy::prelude::*;

use crate::GameState;

pub struct FactoryPlugin;

impl Plugin for FactoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::WorldSpawn),
            systems::initial_spawn_factory,
        )
        .add_systems(Update, systems::clamp_factory_to_cursor_position);
    }
}
