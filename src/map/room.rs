use super::interactable::Interactable;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
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

    pub fn as_collider(&self) -> Collider {
        let (wall_x, wall_y) = self.dimensions();
        Collider::cuboid(wall_x, wall_y)
    }

    // (bottom left, top left, top right, bottom right)
    pub fn _coordinates(&self) -> (Vec2, Vec2, Vec2, Vec2) {
        let dimensions = self.dimensions();
        let x_min = self.position.x - dimensions.0;
        let x_max = self.position.x + dimensions.0;
        let y_min = self.position.y - dimensions.1;
        let y_max = self.position.y + dimensions.1;
        (
            Vec2::new(x_min, y_min),
            Vec2::new(x_min, y_max),
            Vec2::new(x_max, y_max),
            Vec2::new(x_max, y_min),
        )
    }
}

impl Room {
    pub fn _is_closed_structure() -> bool {
        unimplemented!()
    }
}
