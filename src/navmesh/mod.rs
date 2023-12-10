pub mod components;
mod systems;

use self::components::{Navmesh, PathfindRequest, ToggleNavmeshDebug};
use bevy::prelude::*;

pub struct NavmeshPlugin;

impl Plugin for NavmeshPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Navmesh>()
            .init_resource::<ToggleNavmeshDebug>()
            .add_systems(Update, systems::debug_navmesh)
            .add_event::<PathfindRequest>();
    }
}
