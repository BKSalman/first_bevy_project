use bevy::prelude::*;
use crate::TILE_SIZE;
pub struct SpritePlugin;

pub struct SpriteSheet(Handle<TextureAtlas>);

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PreStartup, load_sprite);
    }
}

pub fn spawn_player_sprite(
    commands: &mut Commands,
    sprite: &SpriteSheet,
    index: usize,
    translation: Vec3
) -> Entity {
    let mut sprite1 = TextureAtlasSprite::new(index);
    sprite1.custom_size = Some(Vec2::splat(TILE_SIZE));
    
    commands.spawn_bundle(SpriteSheetBundle {
        sprite: sprite1,
        texture_atlas: sprite.0.clone(),
        transform: Transform {
            translation: translation,
            ..Default::default()
        },
        ..Default::default()
    })
    .id()
}

fn load_sprite(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let image = assets.load("first_sprite_sheet.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(17.0),
        2,
        1,
        Vec2::splat(3.0)
    );
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(SpriteSheet(atlas_handle));
}