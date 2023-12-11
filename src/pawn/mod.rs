mod components;
mod systems;

use self::components::work_order::{BuildItem, WorkOrder};
use crate::GameState;
use bevy::prelude::*;
use std::collections::VecDeque;

pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Main), systems::initial_pawn_spawn)
            .init_resource::<WorkQueue>()
            .add_event::<SpawnPawnRequestEvent>()
            .add_systems(
                Update,
                (
                    systems::listen_for_pathfinding_answers.after(systems::work_idle_pawns),
                    systems::move_pawn.after(systems::listen_for_pathfinding_answers),
                    systems::update_health_ui,
                    systems::update_pawn_animation,
                    systems::mine_stone,
                    systems::return_to_factory,
                    systems::work_idle_pawns
                        .after(systems::return_to_factory)
                        .after(systems::mine_stone),
                ),
            );
    }
}

#[derive(Resource, Default)]
pub struct WorkQueue {
    pub build_queue: VecDeque<WorkOrder<BuildItem>>,
}

#[derive(Event, Debug)]
pub struct SpawnPawnRequestEvent;
