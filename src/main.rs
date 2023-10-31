mod character;
mod player;
mod states;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use character::{AnimationIndices, CharacterMovements};
use player::PlayerBundle;
use states::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
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
        .add_systems(OnEnter(GameState::Playing), spawn_player)
        .add_systems(
            Update,
            animate_sprite_movement.run_if(in_state(GameState::Playing)),
        )
        .run();
}

#[derive(AssetCollection, Resource)]
struct ImageAssets {
    #[asset(key = "image.player")]
    player: Handle<TextureAtlas>,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn spawn_player(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        PlayerBundle::new("Marshall"),
        CharacterMovements::IdleDown,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        SpriteSheetBundle {
            transform: Transform::from_scale(Vec3::splat(6.0)),
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: image_assets.player.clone(),
            ..default()
        },
    ));
}

fn animate_sprite_movement(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &CharacterMovements,
    )>,
) {
    for (mut timer, mut sprite, movement_type) in &mut query {
        let animation_indices = AnimationIndices::from_movement_type(movement_type);
        timer.0.tick(time.delta());
        if timer.0.finished() {
            println!("{}", sprite.index);
            if sprite.index >= animation_indices.1 || sprite.index < animation_indices.0 {
                sprite.index = animation_indices.0
            } else {
                sprite.index += 1;
            }
        }
    }
}
