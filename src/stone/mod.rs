mod components;
mod systems;

use self::systems::spawn_stone_tiles;
use crate::{build_map, GameState};
use bevy::prelude::*;

pub use components::*;

pub struct StonePlugin;

impl Plugin for StonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::WorldSpawn),
            spawn_stone_tiles.after(build_map),
        );
    }
}
