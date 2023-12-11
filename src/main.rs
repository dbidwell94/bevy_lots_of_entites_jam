// This attr removes the console on release builds on Windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod assets;
mod factory;
mod navmesh;
mod pawn;
mod stone;
mod ui;
mod utils;

use assets::{DirtTile, GameAssets, GroundBase};
use bevy::{asset::AssetMetaCheck, prelude::*, window::PrimaryWindow};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_easings::*;
use leafwing_input_manager::{axislike::VirtualAxis, prelude::*};
use noisy_bevy::simplex_noise_2d_seeded;
use rand::prelude::*;
use utils::TranslationHelper;

#[cfg(target_arch = "wasm32")]
const SIZE: usize = 128;
#[cfg(not(target_arch = "wasm32"))]
const SIZE: usize = 128;
const DIRT_CUTOFF: f32 = -1.;
const GRASS_CUTOFF: f32 = 0.0;
const TILE_SIZE: f32 = 16.;
const PERLIN_DIVIDER: f32 = 75.;

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Reflect)]
pub enum GameState {
    #[default]
    Loading,
    WorldSpawn,
    FactoryPlacement,
    Main,
    Paused,
}

#[derive(Actionlike, Reflect, Clone, Hash, PartialEq, Eq, Debug)]
pub enum Input {
    Pan,
    Zoom,
    Select,
    Debug,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::WorldSpawn),
        )
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            DefaultPlugins
                .build()
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#canvas".into()),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
            GameAssets,
        ))
        .add_plugins(InputManagerPlugin::<Input>::default())
        .add_plugins((
            pawn::PawnPlugin,
            stone::StonePlugin,
            factory::FactoryPlugin,
            ui::UIPlugin,
            navmesh::NavmeshPlugin,
        ))
        .add_systems(OnEnter(GameState::WorldSpawn), build_map)
        .add_systems(Update, update_cursor_position)
        .add_systems(
            Update,
            (
                camera_interactions.run_if(
                    in_state(GameState::Main).or_else(in_state(GameState::FactoryPlacement)),
                ),
                selection_gizmo.after(camera_interactions),
            ),
        )
        .init_resource::<WorldNoise>()
        .init_resource::<CursorPosition>()
        .init_resource::<GameResources>()
        .run();
}

#[derive(Component)]
struct CameraMetadata {
    pub target: Vec3,
    pub zoom: f32,
    /// The world position of the mouse when the user started clicking and where the user is dragging to. None if the user is not dragging.
    pub selection_world_bounds: Option<(Vec2, Vec2)>,
}

#[derive(Resource)]
pub struct WorldNoise {
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

#[derive(Resource, Debug, Default)]
pub struct GameResources {
    pub stone: usize,
    pub pawns: usize,
}

#[derive(Resource, Default)]
pub struct CursorPosition(pub Option<Vec2>);

#[derive(Component)]
struct GameTile;

#[derive(Component)]
enum TileType {
    Dirt,
    Grass,
}

pub fn build_map(
    mut commands: Commands,
    mut world_noise: ResMut<WorldNoise>,
    asset_server: Res<AssetServer>,
    dirt_texture: Res<GroundBase>,
    mut navmesh: ResMut<navmesh::components::Navmesh>,
) {
    let mut camera_bundle = Camera2dBundle::default();

    camera_bundle.projection.scale = 0.50;
    camera_bundle.transform.translation = Vec3::new(
        SIZE as f32 * TILE_SIZE / 2.,
        SIZE as f32 * TILE_SIZE / 2.,
        0.,
    );

    commands.spawn((
        CameraMetadata {
            target: camera_bundle.transform.translation,
            zoom: camera_bundle.projection.scale,
            selection_world_bounds: None,
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
                .insert(MouseButton::Left, Input::Select)
                .insert(KeyCode::Grave, Input::Debug)
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
        &mut navmesh,
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
        anchor: bevy::sprite::Anchor::BottomLeft,
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
    navmesh: &mut ResMut<navmesh::components::Navmesh>,
) {
    for x in 0..SIZE {
        for y in 0..SIZE {
            let seed_value = &base_world[x][y];

            let nav_tile = &mut navmesh.0[x][y];

            if seed_value >= &DIRT_CUTOFF && seed_value < &GRASS_CUTOFF {
                // Dirt
                let dirt_entity = commands
                    .spawn((
                        TileType::Dirt,
                        SpriteSheetBundle {
                            sprite: get_dirt_texture_facing_grass(base_world, &x, &y),
                            texture_atlas: dirt_texture.dirt.clone(),
                            transform: Transform::from_translation(
                                Vec2::new(x as f32, y as f32).tile_pos_to_world().extend(0.),
                            ),
                            ..default()
                        },
                        GameTile,
                    ))
                    .id();

                nav_tile.walkable = true;
                nav_tile.weight = 1.;
                nav_tile.occupied_by.insert(dirt_entity);
            }
            if seed_value >= &GRASS_CUTOFF {
                // Grass
                let grass_entity = commands
                    .spawn((
                        TileType::Grass,
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                                anchor: bevy::sprite::Anchor::BottomLeft,
                                ..default()
                            },
                            texture: asset_server.load("grass.png"),
                            transform: Transform::from_translation(
                                Vec2::new(x as f32, y as f32).tile_pos_to_world().extend(0.),
                            ),
                            ..default()
                        },
                        GameTile,
                    ))
                    .id();

                nav_tile.walkable = true;
                nav_tile.weight = 2.;
                nav_tile.occupied_by.insert(grass_entity);
            }
        }
    }
}

fn camera_interactions(
    mut camera_query: Query<
        (
            &mut OrthographicProjection,
            &mut Transform,
            &mut CameraMetadata,
            &Camera,
            &GlobalTransform,
        ),
        With<Camera>,
    >,
    q_window: Query<&Window, With<PrimaryWindow>>,
    input: Query<&ActionState<Input>>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    let Ok((mut projection, mut transform, mut camera_target, camera, global_camera_transform)) =
        camera_query.get_single_mut()
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

    // Check if select is held to indicate a selection drag
    if input.pressed(Input::Select) {
        // get raw current mouse position
        let Ok(window) = q_window.get_single() else {
            return;
        };
        let Some(cursor_position) = window.cursor_position() else {
            return;
        };
        let Some(world_pos) = camera
            .viewport_to_world(global_camera_transform, cursor_position.clone())
            .map(|ray| ray.origin.truncate())
        else {
            return;
        };

        if let Some(bounds) = &mut camera_target.selection_world_bounds {
            // if we already have a selection, update the second point
            bounds.1 = world_pos;
        } else {
            // if we don't have a selection, create one
            camera_target.selection_world_bounds = Some((world_pos, world_pos));
        }
    } else {
        camera_target.selection_world_bounds = None;
    }
}

fn update_cursor_position(
    mut cursor_world_position: ResMut<CursorPosition>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok((camera, camera_transform)) = q_camera.get_single() else {
        return;
    };

    let Ok(window) = q_window.get_single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let world_pos = camera
        .viewport_to_world(camera_transform, cursor_position.clone())
        .map(|ray| ray.origin.truncate());

    cursor_world_position.0 = world_pos.map(|v| {
        Vec2::new(
            ((v.x - TILE_SIZE / 2.) as i32 / TILE_SIZE as i32) as f32,
            ((v.y - TILE_SIZE / 2.) as i32 / TILE_SIZE as i32) as f32,
        )
    });
}

// Show a white box where the user is dragging to select
fn selection_gizmo(mut gizmos: Gizmos, camera_metadata: Query<&CameraMetadata, With<Camera>>) {
    let Ok(camera_metadata) = camera_metadata.get_single() else {
        return;
    };

    if let Some(bounds) = &camera_metadata.selection_world_bounds {
        let (start, end) = bounds;
        let start = start.extend(0.);
        let end = end.extend(0.);

        let min = Vec3::new(start.x.min(end.x), start.y.min(end.y), 0.);
        let max = Vec3::new(start.x.max(end.x), start.y.max(end.y), 0.);

        let position = (min + max) / 2.0;
        let size = max - min;
        let color = Color::WHITE;

        gizmos.rect_2d(position.truncate(), 0., size.truncate(), color);
    }
}
