use super::components::pawn_status::{Idle, Pathfinding, PawnStatus};
use super::components::work_order::{MineStone, WorkOrder};
use super::{EnemyWave, SpawnPawnRequestEvent};
use crate::factory::components::{Factory, Placed};
use crate::navmesh::components::{NavTileOccupant, Navmesh, PathfindAnswer, PathfindRequest};
use crate::navmesh::get_pathing;
use crate::stone::{Stone, StoneKind};
use crate::{
    assets::{CharacterFacing, MalePawns},
    pawn::components::*,
    utils::*,
};
use crate::{GameResources, SIZE, TILE_SIZE};
use bevy::prelude::*;
use bevy::utils::HashSet;
use rand::prelude::*;
use std::collections::VecDeque;

const INITIAL_PAWN_COUNT: usize = 10;
const MOVE_SPEED: f32 = 60.;
const MAX_RESOURCES: usize = 15;
const RESOURCE_GAIN_RATE: usize = 1;
const PAWN_COST: usize = 100;
const ENEMY_TILE_RANGE: usize = 10;

fn spawn_pawn_in_random_location(
    commands: &mut Commands,
    pawn_res: &Res<MalePawns>,
    game_resources: &mut ResMut<GameResources>,
    factory_transform: &GlobalTransform,
    _: &Res<Navmesh>,
) {
    let radius = TILE_SIZE * 5.;
    let mut rng = rand::thread_rng();

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
                mine_timer: Timer::from_seconds(0.5, TimerMode::Once),
                moving: false,
                search_timer: Timer::from_seconds(2.5, TimerMode::Repeating),
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
            pawn_status: PawnStatus(Idle),
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

    game_resources.pawns += 1;
}

pub fn initial_pawn_spawn(
    mut commands: Commands,
    pawn_res: Res<MalePawns>,
    q_factory: Query<&GlobalTransform, (With<Factory>, With<Placed>)>,
    mut game_resources: ResMut<GameResources>,
    navmesh: Res<Navmesh>,
) {
    let Ok(factory_transform) = q_factory.get_single() else {
        return;
    };

    for _ in 0..INITIAL_PAWN_COUNT {
        spawn_pawn_in_random_location(
            &mut commands,
            &pawn_res,
            &mut game_resources,
            &factory_transform,
            &navmesh,
        );
    }
}

pub fn work_idle_pawns(
    mut commands: Commands,
    mut q_pawns: Query<
        (Entity, &Transform),
        (
            With<Pawn>,
            Without<WorkOrder<work_order::ReturnToFactory>>,
            Without<WorkOrder<work_order::MineStone>>,
            Without<PawnStatus<pawn_status::Moving>>,
            With<PawnStatus<Idle>>,
            Without<Enemy>,
        ),
    >,
    q_stones: Query<Entity, With<StoneKind>>,
    navmesh: Res<Navmesh>,
    mut pathfinding_event_writer: EventWriter<PathfindRequest>,
) {
    let navmesh_tiles = &navmesh.0;

    fn check_for_stones(
        entity_set: &HashSet<Entity>,
        q_stones: &Query<Entity, With<StoneKind>>,
    ) -> (bool, Option<Entity>) {
        for entity in entity_set.iter() {
            if q_stones.get(*entity).is_ok() {
                return (true, Some(*entity));
            }
        }
        (false, None)
    }

    for (entity, transform) in &mut q_pawns {
        commands
            .entity(entity)
            .clear_status()
            .insert(PawnStatus(Pathfinding));

        let grid_location = transform.translation.world_pos_to_tile();

        let grid_x = grid_location.x as usize;
        let grid_y = grid_location.y as usize;

        // search the navmesh for non-walkable tiles, and see if the entities within are in q_stones
        let stone_location;
        let stone_entity;
        let mut search_radius: usize = 1;

        // Find the closest stone to the pawn ensuring that the pawn can reach the stone by pathfinding
        'base: loop {
            for x in (grid_x.saturating_sub(search_radius))..=(grid_x + search_radius) {
                for y in (grid_y.saturating_sub(search_radius))..=(grid_y + search_radius) {
                    if let Some(tile) = navmesh_tiles.get(x).and_then(|row| row.get(y)) {
                        let (found, stone_ent) = check_for_stones(&tile.occupied_by, &q_stones);

                        if !tile.walkable
                            && found
                            && get_pathing(
                                PathfindRequest {
                                    start: Vec2::new(x as f32, y as f32),
                                    end: Vec2::new(grid_x as f32, grid_y as f32),
                                    entity,
                                },
                                &navmesh,
                            )
                            .is_some()
                        {
                            stone_entity = stone_ent;
                            stone_location = Some(Vec2::new(x as f32, y as f32));
                            break 'base;
                        }
                    }
                }
            }
            search_radius += 1;
        }
        if let Some(stone_location) = stone_location {
            pathfinding_event_writer.send(PathfindRequest {
                start: grid_location,
                end: stone_location,
                entity,
            });
            commands.entity(entity).insert(WorkOrder(MineStone {
                stone_entity: stone_entity.unwrap(),
            }));
        }
    }
}

pub fn listen_for_pathfinding_answers(
    mut commands: Commands,
    mut answer_events: EventReader<PathfindAnswer>,
    mut q_pawns: Query<&mut Pawn, With<Pawn>>,
) {
    for evt in answer_events.read() {
        let Ok(mut pawn) = q_pawns.get_mut(evt.entity) else {
            continue;
        };

        if let Some(path) = &evt.path {
            pawn.move_path = path.clone().into();
            commands
                .entity(evt.entity)
                .clear_status()
                .insert(PawnStatus(pawn_status::Moving));
        } else {
            commands
                .entity(evt.entity)
                .clear_status()
                .clear_work_order()
                .insert(PawnStatus(pawn_status::PathfindingError));
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

// TODO! Fix this function because it doesn't work properly. But it's not a priority right now.
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
    let green_health_threshold: usize = 75;
    let yellow_health_threshold: usize = 50;
    let red_health_threshold: usize = 25;

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
        } else if pawn.health > green_health_threshold {
            sprite.color = Color::GREEN;
        } else if pawn.health > yellow_health_threshold {
            sprite.color = Color::YELLOW;
        } else if pawn.health > red_health_threshold {
            sprite.color = Color::RED;
        } else {
            sprite.color = Color::rgb(0.5, 0., 0.);
        }
    }
}

pub fn mine_stone(
    mut commands: Commands,
    q_pawns_moving_to_stone: Query<
        (Entity, &Pawn),
        (
            With<PawnStatus<pawn_status::Moving>>,
            With<WorkOrder<MineStone>>,
            Without<PawnStatus<pawn_status::Mining>>,
        ),
    >,
    mut q_pawns: Query<
        (
            Entity,
            &mut Pawn,
            &mut CarriedResources,
            &WorkOrder<MineStone>,
        ),
        (
            With<PawnStatus<pawn_status::Mining>>,
            Without<PawnStatus<pawn_status::Moving>>,
        ),
    >,
    mut q_stones: Query<(Entity, &mut Stone, &Transform), With<StoneKind>>,
    mut navmesh: ResMut<Navmesh>,
    time: Res<Time>,
) {
    // loop through the q_pawns_moving_to_stone to see if any of them have reached their destination.
    // if they have, then we need to set their PawnStatus to Mining.
    for (pawn_entity, pawn) in &q_pawns_moving_to_stone {
        if !pawn.moving {
            commands
                .entity(pawn_entity)
                .clear_status()
                .insert(PawnStatus(pawn_status::Mining));
        }
    }

    for (pawn_entity, mut pawn, mut carried_resources, work_order) in &mut q_pawns {
        if carried_resources.0 >= MAX_RESOURCES {
            commands
                .entity(pawn_entity)
                .clear_work_order()
                .clear_status()
                .insert(PawnStatus(pawn_status::Idle))
                .insert(WorkOrder(work_order::ReturnToFactory));

            continue;
        }

        if pawn.mine_timer.tick(time.delta()).finished() {
            let Ok((stone_entity, mut stone, stone_transform)) =
                q_stones.get_mut(work_order.0.stone_entity)
            else {
                commands
                    .entity(pawn_entity)
                    .clear_status()
                    .clear_work_order()
                    .insert(PawnStatus(Idle));
                continue;
            };

            if stone.remaining_resources > 0 {
                stone.remaining_resources =
                    stone.remaining_resources.saturating_sub(RESOURCE_GAIN_RATE);
                carried_resources.0 = carried_resources.0.saturating_add(RESOURCE_GAIN_RATE);
            } else {
                // we're about to despawn an entity, get it's grid transform and remove it from the navmesh before we despawn it

                let stone_grid = stone_transform.translation.world_pos_to_tile();
                navmesh.0[stone_grid.x as usize][stone_grid.y as usize].walkable = true;
                navmesh.0[stone_grid.x as usize][stone_grid.y as usize]
                    .occupied_by
                    .remove(&stone_entity);

                commands.entity(stone_entity).despawn_recursive();
                commands
                    .entity(pawn_entity)
                    .clear_status()
                    .clear_work_order()
                    .insert(PawnStatus(Idle));
            }
        }
    }
}

pub fn return_to_factory(
    mut commands: Commands,
    q_pawns_need_pathfinding_to_factory: Query<
        (Entity, &Transform),
        (
            With<WorkOrder<work_order::ReturnToFactory>>,
            With<PawnStatus<pawn_status::Idle>>,
            With<Pawn>,
        ),
    >,
    mut q_pawns_moving_to_factory: Query<
        (Entity, &Pawn, &mut CarriedResources),
        (
            With<PawnStatus<pawn_status::Moving>>,
            With<WorkOrder<work_order::ReturnToFactory>>,
            With<Pawn>,
        ),
    >,
    q_factory: Query<&Transform, (With<Factory>, With<Placed>)>,
    mut resources: ResMut<GameResources>,
    mut pathfinding_event_writer: EventWriter<PathfindRequest>,
) {
    let Ok(factory_transform) = q_factory.get_single() else {
        return;
    };

    let factory_grid = factory_transform.translation.world_pos_to_tile();

    // Loop through idle pawns that are looking for the factory
    for (pawn_entity, transform) in &q_pawns_need_pathfinding_to_factory {
        let pawn_location = transform.translation.world_pos_to_tile();

        pathfinding_event_writer.send(PathfindRequest {
            start: pawn_location,
            end: factory_grid,
            entity: pawn_entity,
        });

        commands
            .entity(pawn_entity)
            .clear_status()
            .insert(PawnStatus(pawn_status::Pathfinding));
    }

    // Loop through pawns that are moving to the factory looking for stopped pawns
    // so we can start depositing resources into the factory
    for (pawn_entity, pawn, mut carried_resources) in &mut q_pawns_moving_to_factory {
        if !pawn.moving {
            commands
                .entity(pawn_entity)
                .clear_status()
                .clear_work_order()
                .insert(PawnStatus(pawn_status::Idle));

            resources.stone += carried_resources.0;
            carried_resources.0 = 0;
        }
    }
}

pub fn listen_for_spawn_pawn_event(
    mut commands: Commands,
    pawn_res: Res<MalePawns>,
    q_factory: Query<&GlobalTransform, (With<Factory>, With<Placed>)>,
    mut game_resources: ResMut<GameResources>,
    mut spawn_pawn_event_reader: EventReader<SpawnPawnRequestEvent>,
    navmesh: Res<Navmesh>,
) {
    let Ok(factory_transform) = q_factory.get_single() else {
        return;
    };

    for _ in spawn_pawn_event_reader.read() {
        if game_resources.stone >= PAWN_COST {
            game_resources.stone -= 100;
        } else {
            continue;
        }
        spawn_pawn_in_random_location(
            &mut commands,
            &pawn_res,
            &mut game_resources,
            &factory_transform,
            &navmesh,
        );
    }
}

pub fn debug_pathfinding_error(
    _: Commands,
    q_pawns: Query<Entity, With<PawnStatus<pawn_status::PathfindingError>>>,
) {
    for entity in &q_pawns {
        info!("Pathfinding error for pawn {:?}", entity);
    }
}

pub fn pawn_search_for_enemies(
    mut commands: Commands,
    mut q_pawns: Query<
        (Entity, &mut Pawn, &GlobalTransform),
        (Without<WorkOrder<work_order::AttackPawn>>, Without<Enemy>),
    >,
    q_enemies: Query<(Entity, &GlobalTransform), (With<Enemy>, With<Pawn>)>,
    mut pathfinding_event_writer: EventWriter<PathfindRequest>,
) {
    for (pawn_entity, mut pawn, pawn_transform) in &mut q_pawns {
        let pawn_grid_pos = pawn_transform.translation().world_pos_to_tile();

        let mut closest: Option<(Vec2, Entity)> = None;

        for (enemy_entity, enemy_transform) in &q_enemies {
            // check if the pawn is within range of the enemy
            let enemy_tile_pos = enemy_transform.translation().world_pos_to_tile();

            if (enemy_tile_pos - pawn_grid_pos).length() < ENEMY_TILE_RANGE as f32 {
                // check if the pawn is closer than the current closest pawn
                if let Some((closest_pos, _)) = closest {
                    if (enemy_tile_pos - pawn_grid_pos).length()
                        < (closest_pos - pawn_grid_pos).length()
                    {
                        closest = Some((enemy_tile_pos, enemy_entity));
                    }
                } else {
                    closest = Some((enemy_tile_pos, enemy_entity));
                }
            }
        }

        if let Some((closest_pos, enemy_entity)) = closest {
            commands
                .entity(pawn_entity)
                .clear_work_order()
                .clear_status()
                .insert(WorkOrder(work_order::AttackPawn(enemy_entity)))
                .insert(PawnStatus(pawn_status::Pathfinding));

            pathfinding_event_writer.send(PathfindRequest {
                end: closest_pos,
                entity: pawn_entity,
                start: pawn_grid_pos,
            });
        }
    }
}

pub fn spawn_enemy_pawns(
    mut commands: Commands,
    mut enemy_wave: ResMut<EnemyWave>,
    pawn_res: Res<MalePawns>,
    time: Res<Time>,
    navmesh: Res<Navmesh>,
) {
    enemy_wave.enemy_spawn_timer.tick(time.delta());

    if !enemy_wave.enemy_spawn_timer.finished() {
        return;
    }
    enemy_wave.wave += 1;

    // get a random boolean true or false
    let mut rng = rand::thread_rng();
    let spawn_x = rng.gen_bool(0.5);

    let spawn_location: Vec2;

    loop {
        let temp_location: (usize, usize) = if spawn_x {
            // randomly choose between 0 or SIZE (left or right)
            let x: usize = if rng.gen_bool(0.5) { SIZE } else { 0 };
            let y = rng.gen_range(0..SIZE - 1);

            (x, y)
        } else {
            let x = rng.gen_range(0..SIZE - 1);
            let y: usize = if rng.gen_bool(0.5) { SIZE } else { 0 };
            (x, y)
        };

        // check navtile to ensure it's walkable
        if let Some(NavTileOccupant { walkable, .. }) = navmesh
            .0
            .get(temp_location.0)
            .and_then(|o| o.get(temp_location.1))
        {
            if *walkable {
                spawn_location = Vec2::new(temp_location.0 as f32, temp_location.1 as f32);
                break;
            }
        }
    }

    // convert spawn_location to world coordinates
    let spawn_location = spawn_location.tile_pos_to_world();

    // spawn enemy pawn
    let pawn_entity = commands
        .spawn(PawnBundle {
            pawn: Pawn {
                move_path: VecDeque::new(),
                move_to: None,
                health: 100,
                max_health: 100,
                search_timer: Timer::from_seconds(2.5, TimerMode::Repeating),
                animation_timer: Timer::from_seconds(0.125, TimerMode::Repeating),
                mine_timer: Timer::from_seconds(0.5, TimerMode::Once),
                moving: false,
            },
            character_facing: CharacterFacing::Left,
            name: Name::new("Enemy"),
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: pawn_res.get_random(),
                transform: Transform::from_translation(Vec3::new(
                    spawn_location.x,
                    spawn_location.y,
                    1.,
                )),
                sprite: TextureAtlasSprite {
                    anchor: bevy::sprite::Anchor::BottomLeft,
                    index: CharacterFacing::Left as usize,
                    color: Color::RED,
                    ..default()
                },
                ..Default::default()
            },
            pawn_status: PawnStatus(Idle),
            resources: CarriedResources(0),
        })
        .insert(Enemy)
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

pub fn enemy_search_for_factory(
    mut commands: Commands,
    q_enemy_pawns: Query<
        (Entity, &GlobalTransform),
        (With<Enemy>, With<PawnStatus<pawn_status::Idle>>),
    >,
    q_factory: Query<&GlobalTransform, (With<Factory>, With<Placed>)>,
    mut nav_request: EventWriter<PathfindRequest>,
) {
    let Ok(factory) = q_factory.get_single() else {
        return;
    };

    for (entity, transform) in &q_enemy_pawns {
        let grid_location = transform.translation().world_pos_to_tile();

        nav_request.send(PathfindRequest {
            start: grid_location,
            end: factory.translation().world_pos_to_tile(),
            entity,
        });

        commands
            .entity(entity)
            .clear_status()
            .clear_work_order()
            .insert(PawnStatus(pawn_status::Pathfinding))
            .insert(WorkOrder(work_order::AttackFactory));
    }
}

pub fn enemy_search_for_pawns(
    mut commands: Commands,
    mut q_enemy_pawns: Query<
        (Entity, &GlobalTransform, &mut Pawn),
        (
            With<Enemy>,
            With<PawnStatus<pawn_status::Moving>>,
            Without<WorkOrder<work_order::AttackPawn>>,
        ),
    >,
    q_pawns: Query<(Entity, &GlobalTransform), (With<Pawn>, Without<Enemy>)>,
    time: Res<Time>,
    mut pathfinding_event_writer: EventWriter<PathfindRequest>,
) {
    for (enemy_entity, enemy_transform, mut enemy) in &mut q_enemy_pawns {
        // tick the look timer
        enemy.search_timer.tick(time.delta());

        if !enemy.search_timer.finished() {
            continue;
        }
        let enemy_grid_pos = enemy_transform.translation().world_pos_to_tile();

        let mut closest: Option<(Vec2, Entity)> = None;

        for (pawn_entity, pawn_transform) in &q_pawns {
            // check if the pawn is within range of the enemy
            let pawn_tile_pos = pawn_transform.translation().world_pos_to_tile();

            if (pawn_tile_pos - enemy_grid_pos).length() < ENEMY_TILE_RANGE as f32 {
                // check if the pawn is closer than the current closest pawn
                if let Some((closest_pos, _)) = closest {
                    if (pawn_tile_pos - enemy_grid_pos).length()
                        < (closest_pos - enemy_grid_pos).length()
                    {
                        closest = Some((pawn_tile_pos, pawn_entity));
                    }
                } else {
                    closest = Some((pawn_tile_pos, pawn_entity));
                }
            }
        }

        if let Some((closest_pos, pawn_entity)) = closest {
            commands
                .entity(enemy_entity)
                .clear_work_order()
                .clear_status()
                .insert(WorkOrder(work_order::AttackPawn(pawn_entity)))
                .insert(PawnStatus(pawn_status::Pathfinding));

            pathfinding_event_writer.send(PathfindRequest {
                end: closest_pos,
                entity: enemy_entity,
                start: enemy_grid_pos,
            });
        }
    }
}

pub fn update_pathfinding_to_pawn(mut commands: Commands) {
    
}
