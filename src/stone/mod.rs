mod components;
mod systems;

use crate::{build_map, GameState};
use bevy::prelude::*;

pub use components::*;

pub struct StonePlugin;

impl Plugin for StonePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DropStone>()
            .add_systems(
                OnEnter(GameState::WorldSpawn),
                systems::spawn_stone_tiles.after(build_map),
            )
            .add_systems(
                Update,
                (systems::update_stone_sprite).run_if(in_state(GameState::Main)),
            );
    }
}

#[derive(Event, Debug)]
pub struct DropStone {
    pub stone_kind: StoneKind,
    pub location: Vec2,
    pub amount: usize,
}
