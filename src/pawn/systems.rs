use super::components::pawn_status::{Idle, Pathfinding, PawnStatus};
use crate::factory::components::{Factory, Placed};
use crate::navmesh::components::{Navmesh, PathfindAnswer, PathfindRequest};
use crate::stone::StoneKind;
use crate::TILE_SIZE;
use crate::{
    assets::{CharacterFacing, MalePawns},
    pawn::components::*,
    utils::*,
};
use bevy::prelude::*;
use bevy::utils::HashSet;
use rand::prelude::*;

const INITIAL_PAWN_COUNT: usize = 10;

pub fn initial_pawn_spawn(
    mut commands: Commands,
    pawn_res: Res<MalePawns>,
    q_factory: Query<&GlobalTransform, (With<Factory>, With<Placed>)>,
) {
    let Ok(factory_transform) = q_factory.get_single() else {
        return;
    };

    let radius = TILE_SIZE * 5.;
    let mut rng = rand::thread_rng();

    for _ in 0..INITIAL_PAWN_COUNT {
        let pawn = pawn_res.get_random();

        // spawn pawns in a random circle 1 tile around the factory
        let random_angle: f32 = rng.gen_range(0.0..360.0);
        let x = factory_transform.translation().x + random_angle.cos() * radius;
        let y = factory_transform.translation().y + random_angle.sin() * radius;

        commands.spawn(PawnBundle {
            pawn: Pawn,
            character_facing: CharacterFacing::Left,
            name: Name::new("Pawn"),
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: pawn,
                transform: Transform::from_translation(Vec3::new(x, y, 1.)),
                sprite: TextureAtlasSprite {
                    anchor: bevy::sprite::Anchor::BottomLeft,
                    index: CharacterFacing::Left as usize,
                    ..default()
                },
                ..Default::default()
            },
            pawn_status: pawn_status::PawnStatus(pawn_status::Idle),
            resources: CarriedResources(0),
        });
    }
}

pub fn work_idle_pawns(
    mut commands: Commands,
    mut q_pawns: Query<(Entity, &PawnStatus<Idle>, &CarriedResources, &mut Transform), With<Pawn>>,
    q_stones: Query<Entity, With<StoneKind>>,
    navmesh: Res<Navmesh>,
    mut pathfinding_event_writer: EventWriter<PathfindRequest>,
) {
    let navmesh = &navmesh.0;

    fn check_for_stones(
        entity_set: &HashSet<Entity>,
        q_stones: &Query<Entity, With<StoneKind>>,
    ) -> bool {
        for entity in entity_set.iter() {
            if q_stones.get(*entity).is_ok() {
                return true;
            }
        }
        false
    }

    for (entity, _, _, mut transform) in &mut q_pawns {
        commands
            .entity(entity)
            .remove::<PawnStatus<Idle>>()
            .insert(PawnStatus(Pathfinding));

        let grid_location = transform.translation.world_pos_to_tile();

        let grid_x = grid_location.x as usize;
        let grid_y = grid_location.y as usize;

        // search the navmesh for non-walkable tiles, and see if the entities within are in q_stones
        let mut stone_location = None;

        let mut found_stone = false;
        let mut search_radius: usize = 1;

        'base: while !found_stone {
            for x in (grid_x.saturating_sub(search_radius))..=(grid_x + search_radius) {
                for y in (grid_y.saturating_sub(search_radius))..=(grid_y + search_radius) {
                    if let Some(tile) = navmesh.get(x).and_then(|row| row.get(y)) {
                        if !tile.walkable
                            && check_for_stones(&tile.occupied_by, &q_stones)
                            && !found_stone
                        {
                            found_stone = true;
                            stone_location = Some(Vec2::new(x as f32, y as f32));
                            break 'base;
                        }
                    }
                }
            }
            if !found_stone {
                search_radius += 1;
            }
        }
        if let Some(stone_location) = stone_location {
            pathfinding_event_writer.send(PathfindRequest {
                start: grid_location,
                end: stone_location,
                entity,
            });
        }
    }
}

pub fn listen_for_pathfinding_answers(
    mut answer_events: EventReader<PathfindAnswer>,
    q_pawns: Query<(Entity), With<Pawn>>,
) {
    for evt in answer_events.read() {
        let Ok(pawn) = q_pawns.get(evt.entity) else {
            continue;
        };

        info!("Pawn {:?} got path {:?}", pawn, evt.path);
    }
}
