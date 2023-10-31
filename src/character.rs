use bevy::prelude::*;

#[derive(Component)]
pub struct Character;

#[derive(Debug, Component)]
pub enum CharacterMovements {
    IdleLeft,
    IdleDown,
    IdleUp,
    IdleRight,
    RunLeft,
    RunDown,
    RunUp,
    RunRight,
}

pub struct AnimationIndices(pub usize, pub usize);

impl AnimationIndices {
    pub fn from_movement_type(movement_type: &CharacterMovements) -> Self {
        match movement_type {
            CharacterMovements::IdleDown => AnimationIndices(0, 5),
            CharacterMovements::IdleLeft => AnimationIndices(6, 11),
            CharacterMovements::IdleUp => AnimationIndices(12, 17),
            CharacterMovements::IdleRight => AnimationIndices(18, 23),
            CharacterMovements::RunDown => AnimationIndices(24, 29),
            CharacterMovements::RunLeft => AnimationIndices(30, 35),
            CharacterMovements::RunUp => AnimationIndices(36, 41),
            CharacterMovements::RunRight => AnimationIndices(42, 47),
        }
    }
}
