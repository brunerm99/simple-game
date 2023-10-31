use bevy::prelude::*;

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
}
