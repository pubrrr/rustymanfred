use std::collections::HashMap;

use bevy::asset::Handle;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::DefaultPlugins;

use crate::component::manfred::Manfred;
use crate::component::{Direction, Position, Velocity};

mod component;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_manf.system())
        .add_system(velocity_components.system().label("velocity"))
        .add_system(
            position_components
                .system()
                .label("update_position")
                .after("velocity"),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(manfred_sprite_system.system()),
        )
        .run();
}

fn add_manf(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("images\\manfred_sprite_atlas.png");

    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(40.0, 80.0), 8, 4);

    commands
        .spawn_bundle((Manfred::default(), Position::new(0, 0), Velocity::new()))
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.add(texture_atlas),
            ..Default::default()
        });
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn manfred_sprite_system(mut query: Query<(&mut TextureAtlasSprite, &Manfred, &Velocity)>) {
    if let Some((mut atlas_sprite, manfred, velocity)) = query.iter_mut().next() {
        let new_index = if !velocity.is_moving() {
            0
        } else {
            ((atlas_sprite.index + 1) as usize % 8) as u32
        };

        let direction_offset = match manfred.view_direction {
            Direction::Down => 0,
            Direction::Left => 8,
            Direction::Right => 16,
            Direction::Up => 24,
        };

        atlas_sprite.index = new_index + direction_offset;
    }
}

fn position_components(mut query: Query<(&mut Transform, &Velocity)>) {
    query.for_each_mut(|(mut transform, velocity)| {
        transform.translation.x += velocity.x() as f32;
        transform.translation.y += velocity.y() as f32;
    });
}

fn velocity_components(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Manfred)>,
) {
    if let Some((mut velocity, mut manfred)) = query.iter_mut().next() {
        if keyboard_input.pressed(KeyCode::A) {
            velocity.accelerate(Direction::Left);
        }
        if keyboard_input.pressed(KeyCode::D) {
            velocity.accelerate(Direction::Right);
        }
        if keyboard_input.pressed(KeyCode::W) {
            velocity.accelerate(Direction::Up);
        }
        if keyboard_input.pressed(KeyCode::S) {
            velocity.accelerate(Direction::Down);
        }

        if velocity.is_moving() {
            manfred.view_direction = velocity.get_direction();
        }
    };
}
