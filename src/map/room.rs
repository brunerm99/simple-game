use super::interactable::Interactable;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Resource, Deserialize, Debug)]
pub struct Rooms {
    pub rooms: HashMap<String, Room>,
}

#[derive(Component)]
pub struct RoomComponent;

#[derive(Component)]
pub struct WallComponent;

#[derive(Deserialize, Debug)]
pub struct Room {
    pub interactables: Vec<Interactable>,
    pub walls: Vec<Wall>,
    pub position: Vec2,
}

#[derive(Deserialize, Debug)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Deserialize, Debug)]
pub struct Wall {
    pub name: String,
    pub length: f32,
    pub orientation: Orientation,
    pub position: Vec2,
}

impl Wall {
    pub fn dimensions(&self) -> (f32, f32) {
        match self.orientation {
            Orientation::Horizontal => (self.length, 10.0),
            Orientation::Vertical => (10.0, self.length),
        }
    }
}
