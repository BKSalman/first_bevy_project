use bevy::{prelude::*, transform};
use bevy_inspector_egui::Inspectable;

use crate::sprite::SpriteSheet;
use crate::TILE_SIZE;
use crate::sprite::spawn_player_sprite;
pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App){
        app
        .add_startup_system(spawn_player)
        .add_system(player_movement);

    }
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query.single_mut();

    let total_speed = player.speed * TILE_SIZE * time.delta_seconds();

    if (keyboard.pressed(KeyCode::W) && !(transform.translation.y >= 0.965)) {
        transform.translation.y += total_speed;
    }
    if (keyboard.pressed(KeyCode::S) && !(transform.translation.y <= -0.965)) {
        transform.translation.y -= total_speed;
    }
    if (keyboard.pressed(KeyCode::D) && !(transform.translation.x >= 1.745)) {
        transform.translation.x += total_speed;
    }
    if (keyboard.pressed(KeyCode::A) && !(transform.translation.x <= -1.745)) {
        transform.translation.x -= total_speed;
    }
}

fn spawn_player(mut commands: Commands, sprite: Res<SpriteSheet>) {
    let player = spawn_player_sprite(
        &mut commands,
        &sprite,
        0,
        Vec3::new(0.0, 0.0, 0.0),
    );

    commands.entity(player)
    .insert(Name::new("Player"))
    .insert(Player {
        speed: 10.0
    })
    .id();
}