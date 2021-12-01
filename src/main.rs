use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::DefaultPlugins;

use crate::component::manfred::Manfred;
use crate::component::position::Position;
use crate::system::position::{move_positions_system, update_translations_system, FromXAndY};
use crate::system::velocity::velocity_control_system;
use crate::types::Direction;

mod component;
mod system;
mod types;

const MANFRED_SPRITE_ATLAS_COLUMNS: u32 = 8;

type Velocity = crate::component::velocity::Velocity<10>;

#[derive(SystemLabel, Debug, Eq, PartialEq, Clone, Hash)]
enum SystemLabels {
    ApplyVelocity,
    UpdatePositions,
    UpdateTranslations,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Manfreds Apokalypse".to_string(),
            width: 1600.,
            height: 800.,
            decorations: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_manf.system())
        .add_startup_system(add_tree.system())
        .add_system(
            velocity_control_system
                .system()
                .label(SystemLabels::ApplyVelocity),
        )
        .add_system(
            move_positions_system
                .system()
                .label(SystemLabels::UpdatePositions)
                .after(SystemLabels::ApplyVelocity),
        )
        .add_system(
            update_translations_system
                .system()
                .label(SystemLabels::UpdateTranslations)
                .after(SystemLabels::UpdatePositions),
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
        transform: Transform::from_translation(Vec3::compute_from_x_y(160.0, 160.0)),
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
