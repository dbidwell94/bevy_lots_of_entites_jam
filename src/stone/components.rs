use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum StoneKind {
    CappedRock,
    RedRock,
    SaltRock,
    StoneRock,
    TanRock,
}

#[derive(Component, Debug)]
pub struct Stone {
    pub remaining_resources: usize
}