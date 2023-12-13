pub mod components;
mod systems;

use self::components::work_order::{BuildItem, WorkOrder};
use crate::GameState;
use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(SystemSet, Hash, Debug, Clone, Eq, PartialEq)]
pub enum PawnSystemSet {
    Move,
    Work,
    Attack,
}

pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::PawnSpawn), systems::initial_pawn_spawn)
            .init_resource::<WorkQueue>()
            .init_resource::<EnemyWave>()
            .register_type::<components::Pawn>()
            .add_event::<SpawnPawnRequestEvent>()
            // setup systems scheduling
            .configure_sets(
                Update,
                (
                    PawnSystemSet::Move,
                    PawnSystemSet::Work,
                    PawnSystemSet::Attack,
                )
                    .chain()
                    .run_if(in_state(GameState::Main))
                    .after(crate::navmesh::systems::listen_for_pathfinding_requests),
            )
            // add work systems
            .add_systems(
                Update,
                (
                    systems::mine_stone,
                    systems::work_idle_pawns,
                    systems::return_to_factory,
                )
                    .chain()
                    .in_set(PawnSystemSet::Work),
            )
            // add attack systems
            .add_systems(
                Update,
                (
                    systems::search_for_attack_target_pawn,
                    systems::attack_pawn,
                    systems::update_pathfinding_to_pawn,
                )
                    .chain()
                    .in_set(PawnSystemSet::Attack),
            )
            .add_systems(
                Update,
                (
                    systems::retry_pathfinding,
                    systems::enemy_search_for_factory,
                    systems::listen_for_pathfinding_answers,
                    systems::move_pawn,
                )
                    .chain()
                    .in_set(PawnSystemSet::Move),
            )
            // add general systems
            .add_systems(
                Update,
                (
                    systems::update_health_ui,
                    systems::update_pawn_animation,
                    systems::listen_for_spawn_pawn_event,
                    systems::spawn_enemy_pawns,
                    systems::tick_timers,
                )
                    .chain()
                    .run_if(in_state(GameState::Main)),
            );
    }
}

#[derive(Resource, Default)]
pub struct WorkQueue {
    pub build_queue: VecDeque<WorkOrder<BuildItem>>,
}

#[derive(Event, Debug)]
pub struct SpawnPawnRequestEvent;

#[derive(Resource)]
pub struct EnemyWave {
    pub wave: usize,
    pub enemy_count_multiplier: usize,
    pub enemy_spawn_timer: Timer,
}

impl Default for EnemyWave {
    fn default() -> Self {
        Self {
            wave: 0,
            enemy_count_multiplier: 1,
            enemy_spawn_timer: Timer::from_seconds(30.0, TimerMode::Repeating),
        }
    }
}
