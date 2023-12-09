use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum StoneKind {
    CappedRock,
    RedRock,
    SaltRock,
    StoneRock,
    TanRock,
}
