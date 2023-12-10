use bevy::{prelude::*, utils::hashbrown::HashSet};

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
