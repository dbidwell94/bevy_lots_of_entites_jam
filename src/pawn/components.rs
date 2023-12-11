use crate::assets::CharacterFacing;
use bevy::prelude::*;
pub use pawn_status::ClearStatus;
use std::collections::VecDeque;
pub use work_order::ClearWorkOrder;

#[derive(Component)]
pub struct Pawn {
    pub move_path: VecDeque<Vec2>,
    pub move_to: Option<Vec2>,
    pub health: usize,
    pub max_health: usize,
    pub animation_timer: Timer,
    pub mine_timer: Timer,
    pub search_timer: Timer,
    pub moving: bool,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct HealthBar;

#[derive(Bundle)]
pub struct HealthBundle {
    pub health_bundle: SpriteBundle,
    pub health_bar: HealthBar,
}

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
    use bevy::{ecs::system::EntityCommands, prelude::*};

    pub trait ClearStatus {
        fn clear_status(&mut self) -> &mut Self;
    }

    impl ClearStatus for EntityCommands<'_, '_, '_> {
        fn clear_status(&mut self) -> &mut Self {
            self.remove::<PawnStatus<Idle>>()
                .remove::<PawnStatus<Pathfinding>>()
                .remove::<PawnStatus<PathfindingError>>()
                .remove::<PawnStatus<Moving>>()
                .remove::<PawnStatus<Mining>>()
                .remove::<PawnStatus<Attacking>>();

            self
        }
    }

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
    pub struct PathfindingError;
    impl Status for PathfindingError {}

    #[derive(Component)]
    pub struct Moving;
    impl Status for Moving {}

    #[derive(Component)]
    pub struct Mining;
    impl Status for Mining {}

    #[derive(Component)]
    pub struct Attacking;
    impl Status for Attacking {}
}

pub mod work_order {
    use bevy::{ecs::system::EntityCommands, prelude::*};

    pub trait ClearWorkOrder {
        fn clear_work_order(&mut self) -> &mut Self;
    }

    impl ClearWorkOrder for EntityCommands<'_, '_, '_> {
        fn clear_work_order(&mut self) -> &mut Self {
            self.remove::<WorkOrder<MineStone>>()
                .remove::<WorkOrder<ReturnToFactory>>()
                .remove::<WorkOrder<BuildItem>>()
                .remove::<WorkOrder<AttackPawn>>();

            self
        }
    }

    pub trait OrderItem {}

    #[derive(Component)]
    pub struct WorkOrder<T: Component + OrderItem>(pub T);

    #[derive(Component)]
    pub struct MineStone {
        pub stone_entity: Entity,
    }
    impl OrderItem for MineStone {}

    #[derive(Component)]
    pub struct ReturnToFactory;
    impl OrderItem for ReturnToFactory {}

    #[derive(Component)]
    pub struct BuildItem(pub Entity);
    impl OrderItem for BuildItem {}

    #[derive(Component)]
    pub struct AttackPawn(pub Entity);
    impl OrderItem for AttackPawn {}

    #[derive(Component)]
    pub struct AttackFactory;
    impl OrderItem for AttackFactory {}
}
