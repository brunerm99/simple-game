use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component)]
pub struct InteractableComponent;

#[derive(Deserialize, Debug)]
pub struct Interactable {
    pub name: String,
    pub position: Vec2,
    pub collider_size: Vec2,
}
