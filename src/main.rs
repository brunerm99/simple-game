mod actions;
mod basic_components;
mod character;
mod map;
mod player;
mod states;

use actions::control::move_player;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::*;
use character::{AnimationIndices, CharacterMovement, CharacterMovementOptions, MovementSpeed};
use map::MapPlugin;
use player::{Player, PlayerBundle};
use states::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(MapPlugin)
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Playing),
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            GameState::Loading,
            "characters/dynamic_asset.assets.ron",
        )
        .add_collection_to_loading_state::<_, ImageAssets>(GameState::Loading)
        .insert_resource(Msaa::Off)
        .add_systems(OnEnter(GameState::Playing), (setup_physics, spawn_player))
        .add_systems(
            Update,
            (
                animate_sprite_movement.run_if(in_state(GameState::Playing)),
                move_player.run_if(in_state(GameState::Playing)),
            ),
        )
        .run();
}

// TODO: Remove?
fn setup_physics(mut commands: Commands) {
    // commands
    //     .spawn(RigidBody::Fixed)
    //     .insert(Collider::cuboid(500.0, 10.0))
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));
}

fn spawn_player(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        // TODO: Create more encompassing player bundle?
        PlayerBundle::new("Marshall"),
        Player,
        CharacterMovement(CharacterMovementOptions::IdleDown),
        // Animation
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        SpriteSheetBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: image_assets.player.clone(),
            ..default()
        },
        // Physics
        MovementSpeed(3),
        GravityScale::default(),
        Sleeping::default(),
        Ccd::enabled(),
        RigidBody::KinematicVelocityBased,
        Collider::cuboid(16.0, 23.0), // TODO: Pull from 1/2 of the sprite dimensions in .ron file
        KinematicCharacterController::default(),
    ));
}

fn animate_sprite_movement(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &CharacterMovement,
    )>,
) {
    for (mut timer, mut sprite, movement_type) in &mut query {
        let animation_indices = AnimationIndices::from_movement_type(movement_type);
        timer.0.tick(time.delta());
        if timer.0.finished() {
            if sprite.index >= animation_indices.1 || sprite.index < animation_indices.0 {
                sprite.index = animation_indices.0
            } else {
                sprite.index += 1;
            }
        }
    }
}

#[derive(AssetCollection, Resource)]
struct ImageAssets {
    #[asset(key = "image.player")]
    player: Handle<TextureAtlas>,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);
