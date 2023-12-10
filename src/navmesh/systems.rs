use super::components::*;
use crate::utils::*;
use crate::TILE_SIZE;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

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
            let tile_position = Vec2::new(x as f32, y as f32).tile_pos_to_world() + Vec2::new(TILE_SIZE / 2., TILE_SIZE / 2.);

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
