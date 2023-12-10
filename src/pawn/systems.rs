use crate::factory::components::{Factory, Placed};
use crate::TILE_SIZE;
use crate::{
    assets::{CharacterFacing, MalePawns},
    pawn::components::*,
    GameState,
};
use bevy::prelude::*;
use rand::prelude::*;

const INITIAL_PAWN_COUNT: usize = 10;

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

        commands.spawn(PawnBundle {
            pawn: Pawn,
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
        });
    }
}
