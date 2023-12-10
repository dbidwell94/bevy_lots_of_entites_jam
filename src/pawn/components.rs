use bevy::prelude::*;

use crate::assets::CharacterFacing;

#[derive(Component)]
pub struct Pawn;

#[derive(Bundle)]
pub struct PawnBundle {
    pub character_facing: CharacterFacing,
    pub name: Name,
    pub sprite_bundle: SpriteSheetBundle,
    pub pawn: Pawn,
    pub pawn_status: PawnStatus,
    pub resources: CarriedResources,
}

#[derive(Component)]
pub struct CarriedResources(pub usize);

#[derive(Component, Default)]
pub enum PawnStatus {
    #[default]
    Idle,
    Pathing,
    Mining,
    Returning,
}