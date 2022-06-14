#![allow(unused)]

use bevy::{prelude::*, math::vec2, render::{texture, camera::ScalingMode}};
use arabic_reshaper::arabic_reshape;
extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub const PLAYER_SPRITE: &str = "player.png";
pub const PLAYER_SIZE: (f32, f32) = (112.0, 79.0);
pub const RESOLUTION:f32 = 16.0 / 9.0;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
    .insert_resource(WindowDescriptor {
        title: "lmao".to_string(),
        width: 1280.0,
        height: 720.0,
        // position: Some(Vec2::new(1000.0, 0.0)),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    // .add_startup_system(setup)
    .add_startup_system(hot_reload)
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_player)
    .add_startup_system_to_stage(StartupStage::PreStartup, load_sprite)
    // .add_system(animate_translation)
    .run();
}

#[derive(Component)]
struct AnimateTranslation;

#[derive(Debug)]
#[derive(Component)]
struct ArabicText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/arial.ttf");

    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };

    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(fix_arabic("الو"), text_style.clone(), text_alignment),
            // transform: Transform::from_translation(Vec3::new(100.0, 0.0, 0.0)),
            ..default()
        });

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform::from_translation(Vec3::new(0.0, 100.0, 0.0)),
        ..Default::default()
        });

    // spawn_camera(commands);
    // spawn_player(commands);
}

fn hot_reload(mut commands: Commands, asset_server: Res<AssetServer>) {
    asset_server.watch_for_changes().unwrap();
}

fn spawn_camera (mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    
    camera.orthographic_projection.scaling_mode = ScalingMode::None;
    
    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;
    
    commands.spawn_bundle(camera);
}

fn spawn_player(mut commands: Commands, sprite: Res<SpriteSheet>) {
    let mut sprite1 = TextureAtlasSprite::new(0);
    sprite1.custom_size = Some(Vec2::splat(1.0));
    
    commands.spawn_bundle(SpriteSheetBundle {
        sprite: sprite1,
        texture_atlas: sprite.0.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 900.0),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Name::new("Player"));
}

struct SpriteSheet(Handle<TextureAtlas>);

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

fn animate_translation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateTranslation>)>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.y = 100.0 * time.seconds_since_startup() as f32;
    }
}

fn fix_arabic (text:&str) -> String {

    let edited:String = arabic_reshape(text)
    .graphemes(true)
    .rev()
    .collect();

    println!("{}", edited);

    edited
}
