#![allow(unused)]

use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin, diagnostic::LogDiagnosticsPlugin, render::{texture, camera::ScalingMode}};
use arabic_reshaper::arabic_reshape;
extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

mod player;
mod debug;
mod sprite;

use player::PlayerPlugin;
use debug::DebugPlugin;
use sprite::SpritePlugin;

#[derive(Component)]
struct AnimateTranslation;

#[derive(Debug)]
#[derive(Component)]
struct ArabicText;


pub const PLAYER_SPRITE: &str = "player.png";
pub const PLAYER_SIZE: (f32, f32) = (112.0, 79.0);
pub const RESOLUTION:f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

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
    .add_plugin(PlayerPlugin)
    .add_plugin(SpritePlugin)
    .add_plugin(DebugPlugin)
    // .add_plugin(LogDiagnosticsPlugin::default())
    // .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .run();
}


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
