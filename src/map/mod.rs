mod door;
mod interactable;
mod room;

use crate::basic_components::Name;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use interactable::InteractableComponent;
use log::info;
use ron::de::from_bytes;
use room::{RoomComponent, Rooms, WallComponent};

#[derive(Component)]
pub struct Map;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let rooms = from_bytes::<Rooms>(include_bytes!("../../config/rooms.ron"))
            .expect("Failed to read config file");
        info!("{:#?}", rooms);

        app.insert_resource(rooms)
            .add_systems(OnEnter(GameState::Playing), spawn_rooms);
    }
}

fn spawn_rooms(mut commands: Commands, rooms: Res<Rooms>) {
    // Spawn base map
    commands
        .spawn(Map)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .with_children(|map_parent| {
            // Spawn rooms as children of base map
            rooms.rooms.iter().for_each(|(name, room)| {
                map_parent
                    .spawn(RoomComponent)
                    .insert(Name(name.clone()))
                    .insert(TransformBundle::from(Transform::from_xyz(
                        room.position.x,
                        room.position.y,
                        0.0,
                    )))
                    .with_children(|room_parent| {
                        // Spawn walls as children of room
                        room.walls.iter().for_each(|wall| {
                            let (wall_x, wall_y) = wall.dimensions();
                            room_parent
                                .spawn(WallComponent)
                                .insert(RigidBody::Fixed)
                                .insert(Name(wall.name.clone()))
                                .insert(Collider::cuboid(wall_x, wall_y))
                                .insert(TransformBundle::from(Transform::from_xyz(
                                    wall.position.x,
                                    wall.position.y,
                                    0.0,
                                )));
                        });
                        // Spawn interactables as children of room
                        room.interactables.iter().for_each(|interactable| {
                            room_parent
                                .spawn(InteractableComponent)
                                .insert(RigidBody::Fixed)
                                .insert(Name(interactable.name.clone()))
                                .insert(Collider::cuboid(
                                    interactable.collider_size.x,
                                    interactable.collider_size.y,
                                ))
                                .insert(TransformBundle::from(Transform::from_xyz(
                                    interactable.position.x,
                                    interactable.position.y,
                                    0.0,
                                )));
                        });
                    });
            });
        });
}
