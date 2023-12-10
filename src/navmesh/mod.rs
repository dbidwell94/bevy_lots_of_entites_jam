pub mod components;
mod systems;

use self::components::{Navmesh, PathfindAnswer, PathfindRequest, ToggleNavmeshDebug};
use bevy::prelude::*;

pub struct NavmeshPlugin;

impl Plugin for NavmeshPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Navmesh>()
            .init_resource::<ToggleNavmeshDebug>()
            .add_systems(Update, systems::debug_navmesh)
            .add_systems(Update, systems::listen_for_pathfinding_requests)
            .add_event::<PathfindRequest>()
            .add_event::<PathfindAnswer>();
    }
}
