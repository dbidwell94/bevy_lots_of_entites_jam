use bevy::{prelude::*, utils::HashSet};

use crate::SIZE;

#[derive(Debug, Default, Resource)]
pub struct ToggleNavmeshDebug(pub bool);

#[derive(Debug, Default)]
pub struct NavTileOccupant {
    pub weight: f32,
    pub occupied_by: HashSet<Entity>,
    pub walkable: bool,
}

#[derive(Resource)]
pub struct Navmesh(pub [[NavTileOccupant; SIZE]; SIZE]);

impl Default for Navmesh {
    fn default() -> Self {
        Self(std::array::from_fn(|_| {
            std::array::from_fn(|_| NavTileOccupant::default())
        }))
    }
}

#[derive(Debug, Event)]
pub struct PathfindRequest {
    pub start: Vec2,
    pub end: Vec2,
    pub entity: Entity,
}

#[derive(Debug, Event)]
pub struct PathfindAnswer {
    pub path: Option<Vec<Vec2>>,
    pub entity: Entity,
}
