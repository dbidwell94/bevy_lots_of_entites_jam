use super::components::*;
use crate::utils::*;
use crate::TILE_SIZE;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use pathfinding::prelude::*;

pub fn debug_navmesh(
    navmesh: Res<Navmesh>,
    mut toggle_debug: ResMut<ToggleNavmeshDebug>,
    mut gizmos: Gizmos,
    input: Query<&ActionState<crate::Input>>,
) {
    let Ok(input) = input.get_single() else {
        return;
    };

    if input.just_pressed(crate::Input::Debug) {
        toggle_debug.0 = !toggle_debug.0;
    }

    if !toggle_debug.0 {
        return;
    }

    let max_weight = 2.;

    for (x, row) in navmesh.0.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            let tile_position = Vec2::new(x as f32, y as f32).tile_pos_to_world()
                + Vec2::new(TILE_SIZE / 2., TILE_SIZE / 2.);

            if !tile.walkable {
                gizmos.rect_2d(
                    tile_position,
                    0.,
                    Vec2::new(TILE_SIZE, TILE_SIZE),
                    Color::RED,
                );
            } else {
                let weight_color = Color::rgb(
                    tile.weight / max_weight,
                    tile.weight / max_weight,
                    tile.weight / max_weight,
                );
                gizmos.rect_2d(
                    tile_position,
                    0.,
                    Vec2::new(TILE_SIZE, TILE_SIZE),
                    weight_color,
                );
            }
        }
    }
}

pub fn get_pathing(request: PathfindRequest, navmesh: &Res<Navmesh>) -> Option<Vec<Vec2>> {
    let Vec2 { x, y } = request.start;
    let start_x = x as usize;
    let start_y = y as usize;

    let Vec2 { x: end_x, y: end_y } = request.end;
    let end_x = end_x as usize;
    let end_y = end_y as usize;

    let result = astar(
        &(start_x, start_y),
        |&(x, y)| {
            let up = (x, y.saturating_add(1));
            let down = (x, y.saturating_sub(1));
            let left = (x.saturating_sub(1), y);
            let right = (x.saturating_add(1), y);

            let neighbors = [up, down, left, right]
                .iter()
                .filter(|&(x, y)| {
                    navmesh
                        .0
                        .get(*x)
                        .and_then(|row| row.get(*y))
                        .map(|tile| {
                            tile.walkable
                                || (*x == end_x && *y == end_y)
                                || (*x == start_x && *y == start_y)
                        })
                        .unwrap_or(false)
                })
                .map(|(x, y)| ((*x, *y), 0)) // Modify this line
                .collect::<Vec<_>>();

            neighbors
        },
        |&(x, y)| {
            (Vec2::new(x as f32, y as f32) - Vec2::new(end_x as f32, end_y as f32)).length() as i32
        },
        |(x, y)| x == &end_x && y == &end_y,
    )
    .map(|(data, _)| {
        data.iter()
            .map(|item| Vec2::new(item.0 as f32, item.1 as f32))
            .collect::<Vec<_>>()
    });

    result
}

pub fn listen_for_pathfinding_requests(
    mut pathfinding_event_reader: EventReader<PathfindRequest>,
    navmesh: Res<Navmesh>,
    mut pathfinding_event_writer: EventWriter<PathfindAnswer>,
) {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct UsizeVec {
        x: usize,
        y: usize,
    }
    let navmesh = &navmesh.0;
    for request in pathfinding_event_reader.read() {
        let Vec2 { x, y } = request.start;
        let x = x as usize;
        let y = y as usize;

        let Vec2 { x: end_x, y: end_y } = request.end;
        let end_x = end_x as usize;
        let end_y = end_y as usize;

        let result = astar(
            &UsizeVec { x, y },
            |&UsizeVec { x, y }| {
                let up = (x, y.saturating_add(1));
                let down = (x, y.saturating_sub(1));
                let left = (x.saturating_sub(1), y);
                let right = (x.saturating_add(1), y);

                let neighbors = [up, down, left, right]
                    .iter()
                    .filter(|&(x, y)| {
                        navmesh
                            .get(*x)
                            .and_then(|row| row.get(*y))
                            .map(|tile| tile.walkable || (*x == end_x && *y == end_y))
                            .unwrap_or(false)
                    })
                    .map(|(x, y)| (UsizeVec { x: *x, y: *y }, 0)) // Modify this line
                    .collect::<Vec<_>>();

                neighbors
            },
            |&tile| {
                (Vec2::new(tile.x as f32, tile.y as f32) - Vec2::new(end_x as f32, end_y as f32))
                    .length() as i32
            },
            |UsizeVec { x, y }| x == &end_x && y == &end_y,
        )
        .map(|(data, _)| {
            data.iter()
                .map(|item| Vec2::new(item.x as f32, item.y as f32))
                .collect::<Vec<_>>()
        });

        pathfinding_event_writer.send(PathfindAnswer {
            path: result,
            entity: request.entity,
            target: request.end,
        });
    }
}
