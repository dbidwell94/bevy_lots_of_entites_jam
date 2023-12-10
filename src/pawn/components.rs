use crate::assets::CharacterFacing;
use bevy::prelude::*;

#[derive(Component)]
pub struct Pawn;

#[derive(Bundle)]
pub struct PawnBundle<T: Component + pawn_status::Status> {
    pub character_facing: CharacterFacing,
    pub name: Name,
    pub sprite_bundle: SpriteSheetBundle,
    pub pawn: Pawn,
    pub pawn_status: pawn_status::PawnStatus<T>,
    pub resources: CarriedResources,
}

#[derive(Component)]
pub struct CarriedResources(pub usize);

pub mod pawn_status {
    use bevy::prelude::*;

    pub trait Status {}

    #[derive(Component)]
    pub struct PawnStatus<T: Component + Status>(pub T);

    #[derive(Component)]
    pub struct Idle;
    impl Status for Idle {}

    #[derive(Component)]
    pub struct Pathfinding;
    impl Status for Pathfinding {}

    #[derive(Component)]
    pub struct Moving;
    impl Status for Moving {}

    #[derive(Component)]
    pub struct Mining;
    impl Status for Mining {}

    #[derive(Component)]
    pub struct Returning;
    impl Status for Returning {}
}
