use bevy::prelude::*;

use crate::{
    character::{CharacterMovement, CharacterMovementOptions, MovementSpeed},
    player::Player,
    BASE_MOVEMENT_UNIT,
};

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut CharacterMovement, &MovementSpeed, &mut Transform), With<Player>>,
) {
    let (mut movement_type, speed, mut transform) = query.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        movement_type.0 = CharacterMovementOptions::RunLeft
    } else if keyboard_input.pressed(KeyCode::Right) {
        movement_type.0 = CharacterMovementOptions::RunRight
    } else if keyboard_input.pressed(KeyCode::Up) {
        movement_type.0 = CharacterMovementOptions::RunUp
    } else if keyboard_input.pressed(KeyCode::Down) {
        movement_type.0 = CharacterMovementOptions::RunDown
    } else {
        match movement_type.0 {
            CharacterMovementOptions::IdleLeft | CharacterMovementOptions::RunLeft => {
                movement_type.0 = CharacterMovementOptions::IdleLeft
            }
            CharacterMovementOptions::IdleRight | CharacterMovementOptions::RunRight => {
                movement_type.0 = CharacterMovementOptions::IdleRight
            }
            CharacterMovementOptions::IdleUp | CharacterMovementOptions::RunUp => {
                movement_type.0 = CharacterMovementOptions::IdleUp
            }
            CharacterMovementOptions::IdleDown | CharacterMovementOptions::RunDown => {
                movement_type.0 = CharacterMovementOptions::IdleDown
            }
        }
    }

    match movement_type.0 {
        CharacterMovementOptions::RunLeft => {
            transform.translation.x -= BASE_MOVEMENT_UNIT * (speed.0 as f32) * time.delta_seconds()
        }
        CharacterMovementOptions::RunRight => {
            transform.translation.x += BASE_MOVEMENT_UNIT * (speed.0 as f32) * time.delta_seconds()
        }
        CharacterMovementOptions::RunDown => {
            transform.translation.y -= BASE_MOVEMENT_UNIT * (speed.0 as f32) * time.delta_seconds()
        }
        CharacterMovementOptions::RunUp => {
            transform.translation.y += BASE_MOVEMENT_UNIT * (speed.0 as f32) * time.delta_seconds()
        }
        _ => {}
    }
}
