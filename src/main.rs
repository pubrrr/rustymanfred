use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use std::collections::HashMap;

use crate::component::manfred::Manfred;
use crate::component::Position;
use crate::system::velocity::velocity_control_system;
use crate::types::Direction;

mod component;
mod system;
mod types;

const MANFRED_SPRITE_ATLAS_COLUMNS: u32 = 8;

type Velocity = crate::component::velocity::Velocity<10>;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_manf.system())
        .add_startup_system(add_tree.system())
        .add_system(velocity_control_system.system().label("velocity"))
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
        .spawn_bundle((Manfred::default(), Position::new(0, 0), Velocity::new(5)))
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.add(texture_atlas),
            ..Default::default()
        });
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn add_tree(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("images\\objects\\tree2.png");

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite::new(Vec2::new(80.0, 200.0)),
        material: color_materials.add(ColorMaterial::from(texture_handle)),
        transform: Transform::from_xyz(160.0, 160.0, 10.0),
        ..Default::default()
    });
}

fn manfred_sprite_system(mut query: Query<(&mut TextureAtlasSprite, &Manfred, &Velocity)>) {
    if let Some((mut atlas_sprite, manfred, velocity)) = query.iter_mut().next() {
        let new_index = if !velocity.is_moving() {
            0
        } else {
            ((atlas_sprite.index + 1) % MANFRED_SPRITE_ATLAS_COLUMNS) as u32
        };

        let direction_offset = match manfred.view_direction {
            Direction::Down => 0,
            Direction::Left => 1,
            Direction::Right => 2,
            Direction::Up => 3,
        };

        atlas_sprite.index = new_index + direction_offset * MANFRED_SPRITE_ATLAS_COLUMNS;
    }
}

fn position_components(mut query: Query<(&mut Transform, &Velocity)>) {
    query.for_each_mut(|(mut transform, velocity)| {
        transform.translation.x += velocity.x() as f32;
        transform.translation.y += velocity.y() as f32;
    });
}
