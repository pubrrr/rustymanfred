use bevy::prelude::{Query, Transform, Vec3};

use crate::{Position, Velocity, With};

pub fn move_positions_system(query: Query<(&mut Position, &Velocity)>) {
    query.for_each_mut(|(mut position, velocity)| {
        position.x += velocity.x();
        position.y += velocity.y();
    });
}

pub fn update_translations_system(query: Query<(&mut Transform, &Position), With<Velocity>>) {
    query.for_each_mut(|(mut transform, position)| {
        transform.translation = Vec3::compute_from_x_y(position.x as f32, position.y as f32)
    })
}

pub trait FromXAndY {
    fn compute_from_x_y(x: f32, y: f32) -> Self;
}

impl FromXAndY for Vec3 {
    fn compute_from_x_y(x: f32, y: f32) -> Self {
        // the camera is at z = 1000, the background at z = 0, so put everything somewhere between that
        let z = -y / 10.0 + 500.0;
        Vec3::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use bevy::prelude::{Entity, Transform, Vec3};
    use rstest::*;

    use quickcheck_macros::quickcheck;

    use crate::system::position::{update_translations_system, FromXAndY};
    use crate::{
        move_positions_system, Direction, IntoSystem, ParallelSystemDescriptorCoercion, Position,
        Stage, SystemStage, Velocity, World,
    };

    #[quickcheck]
    fn vec3_from_x_and_y_preserves_x_and_y(x: f32, y: f32) {
        if x.is_finite() && y.is_finite() {
            let under_test = Vec3::compute_from_x_y(x, y);

            assert_eq!(x, under_test.x);
            assert_eq!(y, under_test.y);
        }
    }

    #[rstest]
    #[case::zero_input(vec![], 0.0, 0.0, Ordering::Equal)]
    #[case::down(vec![Direction::Down], 0.0, -1.0, Ordering::Greater)]
    #[case::down_right(vec![Direction::Down, Direction::Right], 1.0, -1.0, Ordering::Greater)]
    #[case::right(vec![Direction::Right], 1.0, 0.0, Ordering::Equal)]
    #[case::up_right(vec![Direction::Up, Direction::Right], 1.0, 1.0, Ordering::Less)]
    #[case::up(vec![Direction::Up],0.0, 1.0, Ordering::Less)]
    #[case::up_left(vec![Direction::Up, Direction::Left], -1.0, 1.0, Ordering::Less)]
    #[case::left_(vec![Direction::Left], -1.0, 0.0, Ordering::Equal)]
    #[case::left_down(vec![Direction::Down, Direction::Left], -1.0, -1.0, Ordering::Greater)]
    fn transform_changes_according_to_velocity(
        #[case] acceleration_steps: Vec<Direction>,
        #[case] expected_x: f32,
        #[case] expected_y: f32,
        #[case] expected_z_relation: Ordering,
    ) {
        let mut world = WorldWrapper::init();

        let (transform, _) = world.get_entity();
        let initial_z = *&transform.translation.z;

        for acceleration in acceleration_steps {
            world.accelerate_entity(acceleration);
        }

        world.run_step();

        let (transform, _) = world.get_entity();
        assert_eq!(expected_x, transform.translation.x);
        assert_eq!(expected_y, transform.translation.y);
        assert_eq!(
            Some(expected_z_relation),
            transform.translation.z.partial_cmp(&initial_z),
            "comparing {} to {}",
            transform.translation.z,
            initial_z
        );
        assert!(transform.translation.z >= 0.0)
    }

    struct WorldWrapper {
        entity_id: Entity,
        world: World,
        system_stage: SystemStage,
    }

    impl WorldWrapper {
        fn init() -> WorldWrapper {
            let mut world = World::default();

            let mut system_stage = SystemStage::parallel();
            system_stage.add_system(move_positions_system.system().label("move"));
            system_stage.add_system(update_translations_system.system().after("move"));

            let entity_id = world
                .spawn()
                .insert(Transform::from_translation(Vec3::compute_from_x_y(
                    0.0, 0.0,
                )))
                .insert(Velocity::new(10))
                .insert(Position::default())
                .id();

            WorldWrapper {
                entity_id,
                world,
                system_stage,
            }
        }

        fn run_step(&mut self) {
            self.system_stage.run(&mut self.world);
        }

        fn get_entity(&self) -> (&Transform, &Velocity) {
            let transform = self.world.get::<Transform>(self.entity_id).unwrap();
            let velocity = self.world.get::<Velocity>(self.entity_id).unwrap();
            (transform, velocity)
        }

        fn accelerate_entity(&mut self, direction: Direction) {
            let mut velocity = self.world.get_mut::<Velocity>(self.entity_id).unwrap();
            velocity.accelerate(direction);
        }
    }
}
