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
use std::collections::VecDeque;

const INITIAL_PAWN_COUNT: usize = 10;
const MOVE_SPEED: f32 = 45.;
const MAX_RESOURCES: usize = 50;

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

        let pawn_entity = commands
            .spawn(PawnBundle {
                pawn: Pawn {
                    move_path: VecDeque::new(),
                    move_to: None,
                    health: 100,
                    max_health: 100,
                    animation_timer: Timer::from_seconds(0.125, TimerMode::Repeating),
                    moving: false,
                },
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
            })
            .id();

        commands
            .spawn(HealthBundle {
                health_bar: HealthBar,
                health_bundle: SpriteBundle {
                    transform: Transform::from_xyz(16. / 2., 20., 1.),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(16., 2.)),
                        color: Color::NONE,
                        ..default()
                    },
                    ..default()
                },
            })
            .set_parent(pawn_entity);
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
    mut q_pawns: Query<&mut Pawn, With<Pawn>>,
) {
    for evt in answer_events.read() {
        let Ok(mut pawn) = q_pawns.get_mut(evt.entity) else {
            continue;
        };

        if let Some(path) = &evt.path {
            pawn.move_path = path.clone().into();
        }
    }
}

pub fn move_pawn(
    mut q_pawn: Query<(&mut Transform, &mut Pawn, &mut CharacterFacing)>,
    time: Res<Time>,
) {
    for (mut transform, mut pawn, mut facing) in &mut q_pawn {
        let current_grid = transform.translation.world_pos_to_tile();

        if pawn.move_to.is_none() {
            pawn.move_to = pawn.move_path.pop_front();
        }

        let Some(path) = pawn.move_to else {
            pawn.moving = false;
            continue;
        };

        let direction = (path - current_grid).normalize_or_zero();

        transform.translation += direction.extend(0.) * MOVE_SPEED * time.delta_seconds();
        pawn.moving = true;
        // update facing direction depending on direction (right, left, forward, backwards)

        if direction.length() > 0. {
            if direction.x.abs() > direction.y.abs() {
                if direction.x > 0. {
                    *facing = CharacterFacing::Right;
                } else {
                    *facing = CharacterFacing::Left;
                }
            } else {
                if direction.y > 0. {
                    *facing = CharacterFacing::Backward;
                } else {
                    *facing = CharacterFacing::Forward;
                }
            }
        }
        if (path - current_grid).length() < 0.2 {
            pawn.move_to = pawn.move_path.pop_front();
        }
    }
}

pub fn update_pawn_animation(
    mut q_pawn: Query<(&mut TextureAtlasSprite, &mut Pawn, &CharacterFacing), With<Pawn>>,
    time: Res<Time>,
) {
    for (mut sprite, mut pawn, facing) in &mut q_pawn {
        if !pawn.moving {
            sprite.index = *facing as usize;
            continue;
        } else {
            pawn.animation_timer.tick(time.delta());
        }

        if pawn.animation_timer.finished() {
            // // step forward 4 cells in the texture atlas to reach the next step in the animation
            // sprite.index += 4;

            let final_animation_frame = 15 - *facing as usize;

            if sprite.index + 4 > final_animation_frame {
                sprite.index = *facing as usize;
            } else {
                sprite.index += 4;
            }
        }
    }
}

pub fn update_health_ui(
    q_pawns: Query<&Pawn>,
    mut q_health_bar: Query<(&Parent, &mut Sprite), With<HealthBar>>,
) {
    let GREEN_HEALTH_THRESHOLD: usize = 75;
    let YELLOW_HEALTH_THRESHOLD: usize = 50;
    let RED_HEALTH_THRESHOLD: usize = 25;

    for (parent, mut sprite) in &mut q_health_bar {
        let pawn_entity = parent.get();

        let Ok(pawn) = q_pawns.get(pawn_entity) else {
            continue;
        };

        sprite.custom_size = Some(Vec2::new(
            pawn.health as f32 / pawn.max_health as f32 * 16.,
            2.,
        ));

        if pawn.health == pawn.max_health {
            sprite.color = Color::NONE;
        } else if pawn.health > GREEN_HEALTH_THRESHOLD {
            sprite.color = Color::GREEN;
        } else if pawn.health > YELLOW_HEALTH_THRESHOLD {
            sprite.color = Color::YELLOW;
        } else if pawn.health > RED_HEALTH_THRESHOLD {
            sprite.color = Color::RED;
        } else {
            sprite.color = Color::rgb(0.5, 0., 0.);
        }
    }
}
