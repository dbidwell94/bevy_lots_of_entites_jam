use crate::{
    assets::{CharacterFacing, MalePawns},
    pawn::components::*,
    GameState,
};
use bevy::prelude::*;

const INITIAL_PAWN_COUNT: usize = 10;

pub fn spawn_initial_pawns(
    mut commands: Commands,
    pawn_res: Res<MalePawns>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for _ in 0..1 {
        let facing = CharacterFacing::Forward;
        commands.spawn(PawnBundle {
            character_facing: facing,
            name: Name::new("Pawn".to_string()),
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: pawn_res.get_random(),
                sprite: TextureAtlasSprite {
                    index: facing as usize,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                ..Default::default()
            },
            pawn: Pawn,
        });
    }

    game_state.set(GameState::Main);
}
