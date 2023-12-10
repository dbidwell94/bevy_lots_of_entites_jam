use super::components::*;
use crate::{CursorPosition, GameState, TILE_SIZE};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

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
                anchor: bevy::sprite::Anchor::BottomCenter,
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

    factory_transform.translation.x = cursor_position.x * TILE_SIZE + (TILE_SIZE / 2.);
    factory_transform.translation.y = cursor_position.y * TILE_SIZE + (TILE_SIZE / 2.);
}

pub fn place_factory(
    mut commands: Commands,
    q_factory: Query<Entity, (With<Factory>, Without<Placed>)>,
    input: Query<&ActionState<crate::Input>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let Ok(factory_entity) = q_factory.get_single() else {
        return;
    };

    let Ok(input) = input.get_single() else {
        return;
    };

    if input.just_pressed(crate::Input::Select) {
        commands.entity(factory_entity).insert(Placed);
        commands.entity(factory_entity).remove::<AabbGizmo>();
        game_state.set(GameState::Main);
    }
}
