use std::ops::Deref;

use bevy::core::CorePlugin;
use bevy::input::InputPlugin;
use bevy::prelude::*;
use bevy::window::WindowPlugin;

use crate::component::manfred::Manfred;
use crate::component::{Position, Velocity};

mod component;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_manf.system())
        .add_system(velocity_components.system())
        .add_system(position_components.system())
        .run();
}

fn add_manf(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("images\\manfred_down0.png");
    commands
        .spawn_bundle((Manfred, Position::new(0, 0), Velocity::new()))
        .insert_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(40.0, 80.0)),
            material: materials.add(texture_handle.into()),
            ..Default::default()
        });
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn position_components(mut query: Query<(&mut Transform, &Velocity)>) {
    query.for_each_mut(|(mut transform, velocity)| {
        transform.translation.x += velocity.x as f32;
        transform.translation.y += velocity.y as f32;
        println!(
            "manfred at ({},{})",
            transform.translation.x, transform.translation.y
        )
    });
}

fn velocity_components(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Manfred>>,
) {
    if let Some(mut velocity) = query.iter_mut().next() {
        if keyboard_input.pressed(KeyCode::A) {
            velocity.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::D) {
            velocity.x += 1;
        }
        if keyboard_input.pressed(KeyCode::W) {
            velocity.y += 1;
        }
        if keyboard_input.pressed(KeyCode::S) {
            velocity.y -= 1;
        }
    };
}
