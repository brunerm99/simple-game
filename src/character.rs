use bevy::prelude::*;

#[derive(Component)]
pub struct Character;

#[derive(Debug, Component)]
pub struct CharacterMovement(pub CharacterMovementOptions);

#[derive(Debug)]
pub enum CharacterMovementOptions {
    IdleLeft,
    IdleDown,
    IdleUp,
    IdleRight,
    RunLeft,
    RunDown,
    RunUp,
    RunRight,
}

#[derive(Component)]
pub struct MovementSpeed(pub u8);

impl Default for MovementSpeed {
    fn default() -> Self {
        MovementSpeed(1)
    }
}

pub struct AnimationIndices(pub usize, pub usize);

impl AnimationIndices {
    pub fn from_movement_type(movement_type: &CharacterMovement) -> Self {
        match movement_type.0 {
            CharacterMovementOptions::IdleDown => AnimationIndices(0, 5),
            CharacterMovementOptions::IdleLeft => AnimationIndices(6, 11),
            CharacterMovementOptions::IdleUp => AnimationIndices(12, 17),
            CharacterMovementOptions::IdleRight => AnimationIndices(18, 23),
            CharacterMovementOptions::RunDown => AnimationIndices(24, 29),
            CharacterMovementOptions::RunLeft => AnimationIndices(30, 35),
            CharacterMovementOptions::RunUp => AnimationIndices(36, 41),
            CharacterMovementOptions::RunRight => AnimationIndices(42, 47),
        }
    }
}
