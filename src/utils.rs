use crate::TILE_SIZE;
use bevy::prelude::*;

pub trait TranslationHelper {
    fn world_pos_to_tile(&self) -> Vec2;
    fn tile_pos_to_world(&self) -> Vec2;
}

impl TranslationHelper for Transform {
    fn world_pos_to_tile(&self) -> Vec2 {
        Vec2::new(
            (self.translation.x / TILE_SIZE).floor(),
            (self.translation.y / TILE_SIZE).floor(),
        )
    }

    fn tile_pos_to_world(&self) -> Vec2 {
        Vec2::new(
            self.translation.x * TILE_SIZE - TILE_SIZE / 2.,
            self.translation.y * TILE_SIZE - TILE_SIZE / 2.,
        )
    }
}

impl TranslationHelper for GlobalTransform {
    fn world_pos_to_tile(&self) -> Vec2 {
        Vec2::new(
            (self.translation().x / TILE_SIZE).floor(),
            (self.translation().y / TILE_SIZE).floor(),
        )
    }

    fn tile_pos_to_world(&self) -> Vec2 {
        Vec2::new(
            self.translation().x * TILE_SIZE - TILE_SIZE / 2.,
            self.translation().y * TILE_SIZE - TILE_SIZE / 2.,
        )
    }
}

impl TranslationHelper for Vec3 {
    fn world_pos_to_tile(&self) -> Vec2 {
        Vec2::new((self.x / TILE_SIZE).floor(), (self.y / TILE_SIZE).floor())
    }

    fn tile_pos_to_world(&self) -> Vec2 {
        Vec2::new(
            self.x * TILE_SIZE - TILE_SIZE / 2.,
            self.y * TILE_SIZE - TILE_SIZE / 2.,
        )
    }
}

impl TranslationHelper for Vec2 {
    fn world_pos_to_tile(&self) -> Vec2 {
        Vec2::new((self.x / TILE_SIZE).floor(), (self.y / TILE_SIZE).floor())
    }

    fn tile_pos_to_world(&self) -> Vec2 {
        Vec2::new(
            self.x * TILE_SIZE + TILE_SIZE / 2.,
            self.y * TILE_SIZE + TILE_SIZE / 2.,
        )
    }
}

#[macro_export]
macro_rules! boxed {
    ($expr: expr) => {
        Box::new($expr)
    };
}
