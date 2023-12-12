use super::{Stone, StoneKind};
use crate::{
    assets::rocks::{RockAsset, RockCollection},
    utils::*,
    GameState, WorldNoise, PERLIN_DIVIDER, SIZE, TILE_SIZE,
};
use bevy::prelude::*;
use noisy_bevy::simplex_noise_2d_seeded;

const MAX_STONE_PER_TILE: usize = 1000;

type StoneGrid = [[Option<StoneKind>; SIZE]; SIZE];

fn get_neighbor_stone_kind(grid: &StoneGrid, x: usize, y: usize) -> Option<StoneKind> {
    // check top
    if y < SIZE - 1 && grid[x][y + 1].is_some() {
        return grid[x][y + 1];
    }
    // check bottom
    if y > 0 && grid[x][y - 1].is_some() {
        return grid[x][y - 1];
    }
    // check left
    if x > 0 && grid[x - 1][y].is_some() {
        return grid[x - 1][y];
    }
    // check right
    if x < SIZE - 1 && grid[x + 1][y].is_some() {
        return grid[x + 1][y];
    }

    // check top left
    if x > 0 && y < SIZE - 1 && grid[x - 1][y + 1].is_some() {
        return grid[x - 1][y + 1];
    }

    // check top right
    if x < SIZE - 1 && y < SIZE - 1 && grid[x + 1][y + 1].is_some() {
        return grid[x + 1][y + 1];
    }

    // check bottom left
    if x > 0 && y > 0 && grid[x - 1][y - 1].is_some() {
        return grid[x - 1][y - 1];
    }

    // check bottom right
    if x < SIZE - 1 && y > 0 && grid[x + 1][y - 1].is_some() {
        return grid[x + 1][y - 1];
    }

    None
}

fn stone_kind_to_resource<'a>(
    stone_kind: StoneKind,
    rock_collection: &'a Res<RockCollection>,
) -> &'a dyn RockAsset {
    match stone_kind {
        StoneKind::CappedRock => &rock_collection.capped_rock,
        StoneKind::RedRock => &rock_collection.red_rock,
        StoneKind::SaltRock => &rock_collection.salt_rock,
        StoneKind::StoneRock => &rock_collection.stone_rock,
        StoneKind::TanRock => &rock_collection.tan_rock,
    }
}

pub fn spawn_stone_tiles(
    mut commands: Commands,
    rock_collection: Res<RockCollection>,
    world_noise: Res<WorldNoise>,
    mut game_state: ResMut<NextState<GameState>>,
    mut navmesh: ResMut<crate::navmesh::components::Navmesh>,
) {
    let mut perlin_location = Vec2::new(0., 0.);

    let mut stone_kinds: StoneGrid = [[Option::<StoneKind>::None; SIZE]; SIZE];

    for x in 0..SIZE {
        for y in 0..SIZE {
            let offset_x = x + world_noise.offset as usize;
            let offset_y = y + world_noise.offset as usize;
            perlin_location.x = offset_x as f32;
            perlin_location.y = offset_y as f32;

            let nav_tile = &mut navmesh.0[x][y];

            let noise_value =
                simplex_noise_2d_seeded(perlin_location / PERLIN_DIVIDER, world_noise.seed);

            if noise_value > 0.70 {
                let noisy_bevy_value =
                    simplex_noise_2d_seeded(perlin_location / 150., world_noise.seed);

                let stone_kind: StoneKind;

                let rock: &dyn RockAsset = if noisy_bevy_value < -0.5 {
                    stone_kind = StoneKind::CappedRock;
                    &rock_collection.capped_rock
                } else if (-0.5..-0.25).contains(&noisy_bevy_value) {
                    stone_kind = StoneKind::RedRock;
                    &rock_collection.red_rock
                } else if (-0.25..0.).contains(&noisy_bevy_value) {
                    stone_kind = StoneKind::SaltRock;
                    &rock_collection.salt_rock
                } else if (0. ..0.25).contains(&noisy_bevy_value) {
                    stone_kind = StoneKind::StoneRock;
                    &rock_collection.stone_rock
                } else {
                    stone_kind = StoneKind::TanRock;
                    &rock_collection.tan_rock
                };

                let (rock, stone_kind) = get_neighbor_stone_kind(&stone_kinds, x, y)
                    .map(|kind| (stone_kind_to_resource(kind, &rock_collection), kind))
                    .unwrap_or((rock, stone_kind));

                stone_kinds[x][y] = Some(stone_kind);

                let stone_entity = commands
                    .spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::WHITE,
                                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                                anchor: bevy::sprite::Anchor::BottomLeft,
                                ..default()
                            },
                            texture: rock.get_large(),
                            transform: Transform::from_translation(
                                Vec2::new(x as f32, y as f32)
                                    .tile_pos_to_world()
                                    .extend(0.5),
                            ),
                            ..default()
                        },
                        stone_kind,
                        Stone {
                            remaining_resources: MAX_STONE_PER_TILE,
                        },
                    ))
                    .id();

                nav_tile.walkable = false;
                nav_tile.occupied_by.insert(stone_entity);
            }
        }
    }

    game_state.set(GameState::FactoryPlacement);
}

pub fn update_stone_sprite(
    mut q_stone: Query<(&Stone, &StoneKind, &mut Handle<Image>, &mut Sprite), Changed<Stone>>,
    rock_collection: Res<RockCollection>,
) {
    for (stone, kind, mut image, mut sprite) in &mut q_stone {
        let stone_resource = stone_kind_to_resource(*kind, &rock_collection);
        let rock_image = if stone.remaining_resources < 250 {
            sprite.custom_size = Some(Vec2::new(TILE_SIZE * 0.5, TILE_SIZE * 0.5));
            stone_resource.get_small()
        } else if stone.remaining_resources < 350 {
            sprite.custom_size = Some(Vec2::new(TILE_SIZE * 0.7, TILE_SIZE * 0.7));
            stone_resource.get_medium_small()
        } else if stone.remaining_resources < 500 {
            sprite.custom_size = Some(Vec2::new(TILE_SIZE * 0.85, TILE_SIZE * 0.85));
            stone_resource.get_medium()
        } else if stone.remaining_resources < 750 {
            sprite.custom_size = Some(Vec2::new(TILE_SIZE * 0.95, TILE_SIZE * 0.95));
            stone_resource.get_medium_large()
        } else {
            sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
            stone_resource.get_large()
        };

        *image = rock_image;
    }
}
