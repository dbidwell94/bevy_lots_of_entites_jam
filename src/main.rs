mod assets;
mod pawn;
mod stone;

use assets::{rocks::RockAsset, DirtTile, GameAssets, GroundBase};
use bevy::{prelude::*, transform::commands};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_easings::*;
use leafwing_input_manager::{axislike::VirtualAxis, prelude::*};
use noisy_bevy::simplex_noise_2d_seeded;
use rand::prelude::*;
use stone::StoneKind;

const SIZE: usize = 256;
const DIRT_CUTOFF: f32 = -0.75;
const GRASS_CUTOFF: f32 = 0.0;
const TILE_SIZE: f32 = 16.;
const PERLIN_DIVIDER: f32 = 75.;

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Reflect)]
pub enum GameState {
    #[default]
    Loading,
    WorldSpawn,
    Main,
    Paused,
}

#[derive(Actionlike, Reflect, Clone, Hash, PartialEq, Eq, Debug)]
pub enum Input {
    Pan,
    Zoom,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::WorldSpawn),
        )
        .add_plugins((
            DefaultPlugins.build().set(ImagePlugin::default_nearest()),
            GameAssets,
        ))
        .add_plugins(InputManagerPlugin::<Input>::default())
        .add_plugins(pawn::PawnPlugin)
        .add_systems(
            OnEnter(GameState::WorldSpawn),
            (build_map, spawn_stone_tiles.after(build_map)),
        )
        .add_systems(
            Update,
            pan_and_zoom_camera.run_if(in_state(GameState::Main)),
        )
        .init_resource::<WorldNoise>()
        .run();
}

#[derive(Component)]
struct CameraSmoothTarget {
    pub target: Vec3,
    pub zoom: f32,
}

#[derive(Resource)]
struct WorldNoise {
    pub base_world: [[f32; SIZE]; SIZE],
    pub base_resources: [[f32; SIZE]; SIZE],
    pub seed: f32,
    pub offset: u16,
}

impl Default for WorldNoise {
    fn default() -> Self {
        Self {
            base_world: [[0.0; SIZE]; SIZE],
            base_resources: [[0.0; SIZE]; SIZE],
            seed: random::<f32>(),
            offset: random::<u16>(),
        }
    }
}

#[derive(Component)]
struct GameTile;

#[derive(Component)]
enum TileType {
    Water,
    Dirt,
    Grass,
}

fn build_map(
    mut commands: Commands,
    mut world_noise: ResMut<WorldNoise>,
    asset_server: Res<AssetServer>,
    dirt_texture: Res<GroundBase>,
) {
    let mut camera_bundle = Camera2dBundle::default();

    camera_bundle.projection.scale = 0.50;
    camera_bundle.transform.translation = Vec3::new(
        SIZE as f32 * TILE_SIZE / 2.,
        SIZE as f32 * TILE_SIZE / 2.,
        0.,
    );

    commands.spawn((
        CameraSmoothTarget {
            target: camera_bundle.transform.translation,
            zoom: camera_bundle.projection.scale,
        },
        camera_bundle,
        InputManagerBundle::<Input> {
            input_map: InputMap::default()
                .insert(
                    VirtualDPad {
                        up: KeyCode::W.into(),
                        down: KeyCode::S.into(),
                        left: KeyCode::A.into(),
                        right: KeyCode::D.into(),
                    },
                    Input::Pan,
                )
                .insert(
                    VirtualAxis {
                        positive: MouseWheelDirection::Up.into(),
                        negative: MouseWheelDirection::Down.into(),
                    },
                    Input::Zoom,
                )
                .build(),
            ..default()
        },
    ));

    let mut perlin_location = Vec2::new(0., 0.);

    for x in 0..SIZE {
        for y in 0..SIZE {
            let offset_x = x + world_noise.offset as usize;
            let offset_y = y + world_noise.offset as usize;
            perlin_location.x = offset_x as f32;
            perlin_location.y = offset_y as f32;

            let noise_value =
                simplex_noise_2d_seeded(perlin_location / PERLIN_DIVIDER, world_noise.seed);
            world_noise.base_world[x][y] = noise_value;

            let noisy_bevy_value =
                simplex_noise_2d_seeded(perlin_location / 100., world_noise.seed);
            world_noise.base_resources[x][y] = noisy_bevy_value;
        }
    }

    spawn_world_tiles(
        &mut commands,
        &world_noise.base_world,
        &asset_server,
        &dirt_texture,
    );
}

fn get_dirt_texture_facing_grass(
    base_world: &[[f32; SIZE]; SIZE],
    x: &usize,
    y: &usize,
) -> TextureAtlasSprite {
    let mut sprite = TextureAtlasSprite {
        index: DirtTile::MiddleMiddle as usize,
        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
        ..default()
    };

    let mut found_grass = false;

    // middle bottom check
    if y > &0 && base_world[*x][*y - 1] >= GRASS_CUTOFF {
        sprite.index = DirtTile::BottomMiddle as usize;
        found_grass = true;
    }
    // middle top check
    if y < &(SIZE - 1) && base_world[*x][*y + 1] >= GRASS_CUTOFF {
        sprite.index = DirtTile::TopMiddle as usize;
        found_grass = true;
    }
    // middle left check
    if x > &0 && base_world[*x - 1][*y] >= GRASS_CUTOFF {
        sprite.index = DirtTile::MiddleLeft as usize;
        found_grass = true;
    }
    // middle right check
    if x < &(SIZE - 1) && base_world[*x + 1][*y] >= GRASS_CUTOFF {
        sprite.index = DirtTile::MiddleRight as usize;
        found_grass = true;
    }

    // left check AND lower check
    if x > &0
        && base_world[*x - 1][*y] >= GRASS_CUTOFF
        && y > &0
        && base_world[*x][*y - 1] >= GRASS_CUTOFF
    {
        sprite.index = DirtTile::BottomLeft as usize;
        found_grass = true;
    }
    // right check AND lower check
    if x < &(SIZE - 1)
        && base_world[*x + 1][*y] >= GRASS_CUTOFF
        && y > &0
        && base_world[*x][*y - 1] >= GRASS_CUTOFF
    {
        sprite.index = DirtTile::BottomRight as usize;
        found_grass = true;
    }
    // left check AND upper check
    if x > &0
        && base_world[*x - 1][*y] >= GRASS_CUTOFF
        && y < &(SIZE - 1)
        && base_world[*x][*y + 1] >= GRASS_CUTOFF
    {
        sprite.index = DirtTile::TopLeft as usize;
        found_grass = true;
    }
    // right check AND upper check
    if x < &(SIZE - 1)
        && base_world[*x + 1][*y] >= GRASS_CUTOFF
        && y < &(SIZE - 1)
        && base_world[*x][*y + 1] >= GRASS_CUTOFF
    {
        sprite.index = DirtTile::TopRight as usize;
        found_grass = true;
    }

    if !found_grass {
        // check top left
        if x > &0 && y < &(SIZE - 1) && base_world[*x - 1][*y + 1] >= GRASS_CUTOFF {
            sprite.index = DirtTile::OutsideTopLeft as usize;
        }
        // check top right
        if x < &(SIZE - 1) && y < &(SIZE - 1) && base_world[*x + 1][*y + 1] >= GRASS_CUTOFF {
            sprite.index = DirtTile::OutsideTopRight as usize;
        }
        // check bottom left
        if x > &0 && y > &0 && base_world[*x - 1][*y - 1] >= GRASS_CUTOFF {
            sprite.index = DirtTile::OutsideBottomLeft as usize;
        }
        // check bottom right
        if x < &(SIZE - 1) && y > &0 && base_world[*x + 1][*y - 1] >= GRASS_CUTOFF {
            sprite.index = DirtTile::OutsideBottomRight as usize;
        }
    }

    sprite
}

fn spawn_world_tiles(
    commands: &mut Commands,
    base_world: &[[f32; SIZE]; SIZE],
    asset_server: &Res<AssetServer>,
    dirt_texture: &Res<GroundBase>,
) {
    for x in 0..SIZE {
        for y in 0..SIZE {
            let seed_value = &base_world[x][y];

            if seed_value < &DIRT_CUTOFF {
                // Water
                commands.spawn((
                    TileType::Water,
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::Rgba {
                                red: 0.253,
                                green: 0.41,
                                blue: 0.878,
                                alpha: 1.,
                            },
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..default()
                        },
                        texture: asset_server.load("water.png"),
                        transform: Transform::from_xyz(
                            x as f32 * TILE_SIZE,
                            y as f32 * TILE_SIZE,
                            0.,
                        ),
                        ..default()
                    },
                    GameTile,
                ));
            }

            if seed_value >= &DIRT_CUTOFF && seed_value < &GRASS_CUTOFF {
                // Dirt
                commands.spawn((
                    TileType::Dirt,
                    SpriteSheetBundle {
                        sprite: get_dirt_texture_facing_grass(base_world, &x, &y),
                        texture_atlas: dirt_texture.dirt.clone(),
                        transform: Transform::from_xyz(
                            x as f32 * TILE_SIZE,
                            y as f32 * TILE_SIZE,
                            0.,
                        ),
                        ..default()
                    },
                    GameTile,
                ));
            }
            if seed_value >= &GRASS_CUTOFF {
                // Grass
                commands.spawn((
                    TileType::Grass,
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..default()
                        },
                        texture: asset_server.load("grass.png"),
                        transform: Transform::from_xyz(
                            x as f32 * TILE_SIZE,
                            y as f32 * TILE_SIZE,
                            0.,
                        ),
                        ..default()
                    },
                    GameTile,
                ));
            }
        }
    }
}

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

fn spawn_stone_tiles(
    mut commands: Commands,
    capped_rock: Res<assets::rocks::CappedRock>,
    red_rock: Res<assets::rocks::RedRock>,
    salt_rock: Res<assets::rocks::SaltRock>,
    stone_rock: Res<assets::rocks::StoneRock>,
    tan_rock: Res<assets::rocks::TanRock>,
    world_noise: Res<WorldNoise>,
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
                    GameTile,
                    stone_kind,
                ));
            }
        }
    }
}

fn pan_and_zoom_camera(
    mut camera_query: Query<
        (
            &mut OrthographicProjection,
            &mut Transform,
            &mut CameraSmoothTarget,
        ),
        With<Camera>,
    >,
    input: Query<&ActionState<Input>>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    let Ok((mut projection, mut transform, mut camera_target)) = camera_query.get_single_mut()
    else {
        return;
    };

    let Ok(input) = input.get_single() else {
        return;
    };

    let Some(camera_movement) = input.axis_pair(Input::Pan) else {
        return;
    };

    let camera_movement = camera_movement.xy().normalize_or_zero();
    let camera_zoom = -input.clamped_value(Input::Zoom) * 0.125;

    camera_target.target += camera_movement.extend(0.) * delta * 1000. * projection.scale;
    camera_target
        .target
        .clamp(Vec3::ZERO, Vec3::new(SIZE as f32, SIZE as f32, 0.));
    camera_target.zoom += camera_zoom * delta * 100.;
    camera_target.zoom = camera_target.zoom.clamp(0.1, 2.5);

    transform.translation = transform
        .translation
        .lerp(camera_target.target, 10. * delta);

    projection.scale = projection.scale.lerp(&camera_target.zoom, &(10. * delta));
}