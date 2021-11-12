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

fn add_manf(mut commands: Commands) {
    commands.spawn_bundle((Manfred, Position::new(0, 0), Velocity::new()));
}

fn position_components(mut query: Query<(&mut Position, &Velocity), With<Manfred>>) {
    println!("position query arrived: {}", query.iter_mut().count());
    query.for_each_mut(|(mut position, velocity)| {
        position.x += velocity.x;
        position.y += velocity.y;
        println!("manfred at ({},{})", position.x, position.y)
    });
}

fn velocity_components(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Manfred>>,
) {
    query.for_each_mut(|mut velocity| {
        if keyboard_input.pressed(KeyCode::A) {
            velocity.x -= 1;
        }
    });
}
