use super::StoneKind;
use crate::{
    assets::{self, rocks::RockAsset},
    GameState, WorldNoise, PERLIN_DIVIDER, SIZE, TILE_SIZE,
};
use bevy::prelude::*;
use noisy_bevy::simplex_noise_2d_seeded;

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
    capped_rock: &'a Res<assets::rocks::CappedRock>,
    red_rock: &'a Res<assets::rocks::RedRock>,
    salt_rock: &'a Res<assets::rocks::SaltRock>,
    stone_rock: &'a Res<assets::rocks::StoneRock>,
    tan_rock: &'a Res<assets::rocks::TanRock>,
) -> Box<&'a dyn RockAsset> {
    match stone_kind {
        StoneKind::CappedRock => Box::new(capped_rock.as_ref()),
        StoneKind::RedRock => Box::new(red_rock.as_ref()),
        StoneKind::SaltRock => Box::new(salt_rock.as_ref()),
        StoneKind::StoneRock => Box::new(stone_rock.as_ref()),
        StoneKind::TanRock => Box::new(tan_rock.as_ref()),
    }
}

pub fn spawn_stone_tiles(
    mut commands: Commands,
    capped_rock: Res<assets::rocks::CappedRock>,
    red_rock: Res<assets::rocks::RedRock>,
    salt_rock: Res<assets::rocks::SaltRock>,
    stone_rock: Res<assets::rocks::StoneRock>,
    tan_rock: Res<assets::rocks::TanRock>,
    world_noise: Res<WorldNoise>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let mut perlin_location = Vec2::new(0., 0.);

    let mut stone_kinds: StoneGrid = [[Option::<StoneKind>::None; SIZE]; SIZE];

    for x in 0..SIZE {
        for y in 0..SIZE {
            let offset_x = x + world_noise.offset as usize;
            let offset_y = y + world_noise.offset as usize;
            perlin_location.x = offset_x as f32;
            perlin_location.y = offset_y as f32;

            let noise_value =
                simplex_noise_2d_seeded(perlin_location / PERLIN_DIVIDER, world_noise.seed);

            if noise_value > 0.70 {
                let noisy_bevy_value =
                    simplex_noise_2d_seeded(perlin_location / 150., world_noise.seed);

                let stone_kind: StoneKind;

                let rock: &dyn RockAsset = if noisy_bevy_value < -0.5 {
                    stone_kind = StoneKind::CappedRock;
                    capped_rock.as_ref()
                } else if noisy_bevy_value >= -0.5 && noisy_bevy_value < -0.25 {
                    stone_kind = StoneKind::RedRock;
                    red_rock.as_ref()
                } else if noisy_bevy_value >= -0.25 && noisy_bevy_value < 0. {
                    stone_kind = StoneKind::SaltRock;
                    salt_rock.as_ref()
                } else if noisy_bevy_value >= 0. && noisy_bevy_value < 0.25 {
                    stone_kind = StoneKind::StoneRock;
                    stone_rock.as_ref()
                } else {
                    stone_kind = StoneKind::TanRock;
                    tan_rock.as_ref()
                };

                let (rock, stone_kind) = get_neighbor_stone_kind(&stone_kinds, x, y)
                    .map(|kind| {
                        (
                            stone_kind_to_resource(
                                kind,
                                &capped_rock,
                                &red_rock,
                                &salt_rock,
                                &stone_rock,
                                &tan_rock,
                            ),
                            kind,
                        )
                    })
                    .unwrap_or((Box::new(rock), stone_kind));

                stone_kinds[x][y] = Some(stone_kind);

                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..default()
                        },
                        texture: rock.get_large(),
                        transform: Transform::from_xyz(
                            x as f32 * TILE_SIZE,
                            y as f32 * TILE_SIZE,
                            0.5,
                        ),
                        ..default()
                    },
                    stone_kind,
                ));
            }
        }
    }

    game_state.set(GameState::FactoryPlacement);
}
