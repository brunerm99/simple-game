use crate::character::Character;
use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub hp: i8,
}

impl Health {
    pub fn new() -> Self {
        Health { hp: 100 }
    }
}

#[derive(Component)]
pub struct PlayerBundle {
    pub name: String,
    pub health: Health,
    pub character: Character, // Just for querying
}

impl PlayerBundle {
    pub fn new(name: &str) -> Self {
        PlayerBundle {
            name: name.to_owned(),
            health: Health::new(),
            character: Character,
        }
    }
}
