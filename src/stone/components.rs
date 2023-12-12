use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum StoneKind {
    Capped,
    Red,
    Salt,
    Stone,
    Tan,
}

#[derive(Component, Debug)]
pub struct Stone {
    pub remaining_resources: usize,
}
