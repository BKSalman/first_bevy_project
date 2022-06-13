#![allow(unused)]

use bevy::{prelude::*};

const PLAYER_SPRITE: &str = "player.png";
const PLAYER_SIZE: (f32, f32) = (112.0, 79.0);


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
    .add_startup_system(setup)
    // .add_system(animate_translation)
    .run();
}

#[derive(Component)]
struct AnimateTranslation;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("peepoHappy", text_style.clone(), text_alignment),
            // transform: Transform::from_translation(Vec3::new(100.0, 0.0, 0.0)),
            ..default()
        })
        .insert(AnimateTranslation);
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform::from_translation(Vec3::new(0.0, 100.0, 0.0)),
        ..Default::default()
        });
    commands.spawn().insert(Person).insert(Name("Marina".to_string()));
}

fn animate_translation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateTranslation>)>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.y = 100.0 * time.seconds_since_startup() as f32;
    }
}