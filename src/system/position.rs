use bevy::prelude::{Query, Transform};

use crate::Velocity;

pub fn move_positions_system(mut query: Query<(&mut Transform, &Velocity)>) {
    query.for_each_mut(|(mut transform, velocity)| {
        transform.translation.x += velocity.x() as f32;
        transform.translation.y += velocity.y() as f32;
    });
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{Entity, Transform};
    use rstest::*;

    use crate::{
        move_positions_system, velocity_control_system, Direction, Input, IntoSystem, KeyCode,
        Manfred, Stage, SystemStage, Velocity, World,
    };

    #[rstest]
    #[case(vec![], (0.0, 0.0))]
    #[case(vec![Direction::Down], (0.0, -1.0))]
    #[case(vec![Direction::Down, Direction::Right], (1.0, -1.0))]
    #[case(vec![Direction::Right], (1.0, 0.0))]
    #[case(vec![Direction::Up, Direction::Right], (1.0, 1.0))]
    #[case(vec![Direction::Up], (0.0, 1.0))]
    #[case(vec![Direction::Up, Direction::Left], (-1.0, 1.0))]
    #[case(vec![Direction::Left], (-1.0, 0.0))]
    #[case(vec![Direction::Down, Direction::Left], (-1.0, -1.0))]
    fn transform_changes_according_to_velocity(
        #[case] acceleration_steps: Vec<Direction>,
        #[case] expected_position: (f32, f32),
    ) {
        let mut world = WorldWrapper::init();

        for acceleration in acceleration_steps {
            world.accelerate_entity(acceleration);
        }

        world.run_step();

        let (transform, _) = world.get_entity();
        assert_eq!(expected_position.0, transform.translation.x);
        assert_eq!(expected_position.1, transform.translation.y);
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
            system_stage.add_system(move_positions_system.system());

            let entity_id = world
                .spawn()
                .insert(Transform::identity())
                .insert(Velocity::new(10))
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
