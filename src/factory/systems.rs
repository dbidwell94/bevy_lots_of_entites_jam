use super::components::*;
use crate::{navmesh, CursorPosition, GameState, GameTile, TILE_SIZE};
use crate::{utils::*, SIZE};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

const FACTORY_SIZE: usize = 4;

pub fn initial_spawn_factory(
    mut commands: Commands,
    cursor_position: Res<CursorPosition>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("factory.png"),
            transform: Transform::from_translation(
                cursor_position.0.unwrap_or(Vec2::ZERO).extend(1.),
            ),
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::BottomLeft,
                ..default()
            },
            ..Default::default()
        },
        AabbGizmo {
            color: Some(Color::WHITE),
            ..default()
        },
        Factory,
    ));
}

pub fn clamp_factory_to_cursor_position(
    mut factory_query: Query<&mut Transform, (With<Factory>, Without<Placed>)>,
    cursor_position: Res<CursorPosition>,
) {
    let Ok(mut factory_transform) = factory_query.get_single_mut() else {
        return;
    };

    let Some(cursor_position) = cursor_position.0 else {
        return;
    };

    factory_transform.translation = (cursor_position.tile_pos_to_world()).extend(1.);
}

pub fn place_factory(
    mut commands: Commands,
    q_factory: Query<(Entity, &GlobalTransform), (With<Factory>, Without<Placed>)>,
    input: Query<&ActionState<crate::Input>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut navmesh: ResMut<navmesh::components::Navmesh>,
) {
    let Ok((factory_entity, factory_transform)) = q_factory.get_single() else {
        return;
    };

    let Ok(input) = input.get_single() else {
        return;
    };

    if input.just_pressed(crate::Input::Select) {
        let Vec2 { x, y } = factory_transform.translation().world_pos_to_tile();

        if !check_spawn_bounds_by_navtiles(&navmesh, x, y) {
            return;
        }

        commands.entity(factory_entity).insert((Placed, GameTile));
        commands.entity(factory_entity).remove::<AabbGizmo>();
        game_state.set(GameState::Main);

        // mark navmesh tiles as occupied
        for x in x as usize..x as usize + FACTORY_SIZE {
            for y in y as usize..y as usize + FACTORY_SIZE {
                navmesh.0[x][y].walkable = false;
            }
        }
    }
}

fn check_spawn_bounds_by_navtiles(navmesh: &navmesh::components::Navmesh, x: f32, y: f32) -> bool {
    let mut is_valid = true;

    // check navmesh bounds for non-walkable tiles assuming the factory is anchored in the bottom left
    for x in x as usize..x as usize + FACTORY_SIZE {
        for y in y as usize..y as usize + FACTORY_SIZE {
            if !navmesh.0[x][y].walkable {
                is_valid = false;
            }
        }
    }

    is_valid
}
