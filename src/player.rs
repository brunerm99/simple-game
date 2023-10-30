use crate::character::Character;
use bevy::prelude::*;

#[derive(Resource)]
pub enum CharacterMovements {
    IdleLeft,
    IdleDown,
    IdleUp,
    IdleRight,
    WalkLeft,
    WalkDown,
    WalkUp,
    WalkRight,
    RunLeft,
    RunDown,
    RunUp,
    RunRight,
}

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
    pub sprite: SpriteSheetBundle,
    pub character: Character,
}

impl PlayerBundle {
    pub fn new(
        name: &str,
        sprite_sheet_path: &str,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        let texture_handle = asset_server.load(sprite_sheet_path);
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 46.0), 6, 8, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        PlayerBundle {
            name: name.to_owned(),
            health: Health::new(),
            character: Character {},
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0), // TODO: Change to default player state
                transform: Transform::from_scale(Vec3::splat(6.0)),
                ..default()
            },
        }
    }
}
