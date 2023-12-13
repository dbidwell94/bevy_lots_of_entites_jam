use crate::assets::CharacterFacing;
use bevy::prelude::*;
pub use pawn_status::ClearStatus;
use std::collections::VecDeque;
pub use work_order::ClearWorkOrder;

#[derive(Component, Reflect)]
pub struct Pawn {
    pub move_path: VecDeque<Vec2>,
    pub move_to: Option<Vec2>,
    pub health: usize,
    pub max_health: usize,
    pub animation_timer: Timer,
    pub mine_timer: Timer,
    pub search_timer: Timer,
    pub retry_pathfinding_timer: Timer,
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

    macro_rules! pawn_status {
        ($($name:ident),*) => {
            $(
                #[derive(Component)]
                pub struct $name;
                impl Status for $name {}
            )*
            pub trait ClearStatus {
                fn clear_status(&mut self) -> &mut Self;
            }
            impl ClearStatus for EntityCommands<'_, '_, '_> {
                fn clear_status(&mut self) -> &mut Self {
                    $(
                        self.remove::<PawnStatus<$name>>();
                    )*
                    self
                }
            }
            pub fn register_trait_queryables(app: &mut App) {
                use bevy_trait_query::RegisterExt;
                $(
                    app.register_component_as::<dyn Status, $name>();
                )*
            }
        }
    }

    #[bevy_trait_query::queryable]
    pub trait Status {}

    #[derive(Component)]
    pub struct PawnStatus<T: Component + Status>(pub T);

    pawn_status!(
        Idle,
        Pathfinding,
        PathfindingError,
        Moving,
        Mining,
        Attacking
    );
}

pub mod work_order {
    use bevy::{ecs::system::EntityCommands, prelude::*};

    macro_rules! work_orders {
        (
            $(struct $name: ident {
            $(
                $field: ident: $ty: ty
            ),* $(,)?
            }),*
    ) => {
            $(
                #[derive(Component)]
                pub struct $name {
                    $(
                        pub $field: $ty
                    ),*
                }
                impl OrderItem for $name {
                    fn to_struct(&self) -> OrderType {
                        let any_self = self as &dyn std::any::Any;
                        let self_obj = any_self.downcast_ref::<$name>().unwrap();

                        OrderType::$name(self_obj)
                    }
                }
            )*

            pub enum OrderType<'a> {
                $(
                    $name(&'a $name),
                )*
            }

            pub trait ClearWorkOrder {
                fn clear_work_order(&mut self) -> &mut Self;
            }

            pub trait AddWorkOrder {
                fn add_work_order<T: 'static + OrderItem>(&mut self, order: T) -> &mut Self;
            }

            impl AddWorkOrder for EntityCommands<'_, '_, '_> {
                fn add_work_order<T: 'static + OrderItem>(&mut self, order: T) -> &mut Self {
                    self.clear_work_order();
                    self.try_insert(WorkOrder(Box::new(order)))
                }
            }

            impl ClearWorkOrder for EntityCommands<'_, '_, '_> {
                fn clear_work_order(&mut self) -> &mut Self {
                    $(
                        self.remove::<WorkOrder<$name>>();
                    )*
                    self
                }
            }
        };
    }

    pub trait OrderItem: Sync + Send {
        fn to_struct(&self) -> OrderType;
    }

    pub trait Queueable: OrderItem {}

    #[derive(Component)]
    pub struct WorkOrder<T: OrderItem + ?Sized>(pub Box<T>);

    work_orders!(
        struct MineStone {
            stone_entity: Entity,
        },
        struct ReturnToFactory {},
        struct BuildItem {
            item_entity: Entity,
        },
        struct AttackPawn {
            pawn_entity: Entity,
        },
        struct AttackFactory {}
    );
}
