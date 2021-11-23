use bevy::prelude::{Input, KeyCode, Query, Res};

use crate::{Direction, Manfred, Velocity};

pub fn velocity_control_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Manfred)>,
) {
    if let Some((mut velocity, mut manfred)) = query.iter_mut().next() {
        handle_acceleration(keyboard_input, &mut velocity, &mut manfred)
    };
}

fn handle_acceleration(
    keyboard_input: Res<Input<KeyCode>>,
    velocity: &mut Velocity,
    manfred: &mut Manfred,
) {
    match keyboard_input.pressed(KeyCode::A) {
        true => velocity.accelerate(Direction::Left),
        false => velocity.decelerate(Direction::Left),
    }

    match keyboard_input.pressed(KeyCode::D) {
        true => velocity.accelerate(Direction::Right),
        false => velocity.decelerate(Direction::Right),
    }

    match keyboard_input.pressed(KeyCode::W) {
        true => velocity.accelerate(Direction::Up),
        false => velocity.decelerate(Direction::Up),
    }

    match keyboard_input.pressed(KeyCode::S) {
        true => velocity.accelerate(Direction::Down),
        false => velocity.decelerate(Direction::Down),
    }

    if velocity.is_moving() {
        manfred.view_direction = velocity.get_direction();
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::{
        velocity_control_system, Direction, Input, IntoSystem, KeyCode, Manfred, Stage,
        SystemStage, Velocity, World,
    };

    #[test]
    fn when_no_key_pressed_then_does_not_move() {
        let mut world = WorldWrapper::init();

        world.run_step();

        let (manfred, velocity) = world.get_manfred_entity();
        assert!(!velocity.is_moving());
        assert_eq!(Direction::Down, manfred.view_direction);
    }

    #[test]
    fn when_a_pressed_then_accelerates_left() {
        let mut world = WorldWrapper::init();

        world.given_key_pressed(KeyCode::A);

        world.run_step();

        let (manfred, velocity) = world.get_manfred_entity();
        let velocity_after_step_1 = velocity.clone();
        assert!(velocity_after_step_1.x() < 0);
        assert_eq!(0, velocity_after_step_1.y());
        assert_eq!(Direction::Left, manfred.view_direction);

        world.run_step();

        let (manfred, velocity) = world.get_manfred_entity();
        assert!(velocity.x() < velocity_after_step_1.x());
        assert_eq!(0, velocity.y());
        assert_eq!(Direction::Left, manfred.view_direction);
    }

    #[test]
    fn when_d_pressed_then_accelerates_right() {
        let mut world = WorldWrapper::init();

        world.given_key_pressed(KeyCode::D);

        world.run_step();

        let (manfred, velocity) = world.get_manfred_entity();
        let velocity_after_step_1 = velocity.clone();
        assert!(velocity_after_step_1.x() > 0);
        assert_eq!(0, velocity_after_step_1.y());
        assert_eq!(Direction::Right, manfred.view_direction);

        world.run_step();

        let (manfred, velocity) = world.get_manfred_entity();
        assert!(velocity.x() > velocity_after_step_1.x());
        assert_eq!(0, velocity.y());
        assert_eq!(Direction::Right, manfred.view_direction);
    }

    #[test]
    fn when_s_pressed_then_accelerates_downwards() {
        let mut world = WorldWrapper::init();

        world.given_key_pressed(KeyCode::S);

        world.run_step();

        let (manfred, velocity) = world.get_manfred_entity();
        let velocity_after_step_1 = velocity.clone();
        assert_eq!(0, velocity_after_step_1.x());
        assert!(velocity_after_step_1.y() < 0);
        assert_eq!(Direction::Down, manfred.view_direction);

        world.run_step();

        let (manfred, velocity) = world.get_manfred_entity();
        assert_eq!(0, velocity.x());
        assert!(velocity.y() < velocity_after_step_1.y());
        assert_eq!(Direction::Down, manfred.view_direction);
    }

    #[test]
    fn when_w_pressed_then_accelerates_upwards() {
        let mut world = WorldWrapper::init();

        world.given_key_pressed(KeyCode::W);

        world.run_step();

        let (manfred, velocity) = world.get_manfred_entity();
        let velocity_after_step_1 = velocity.clone();
        assert_eq!(0, velocity_after_step_1.x());
        assert!(velocity_after_step_1.y() > 0);
        assert_eq!(Direction::Up, manfred.view_direction);

        world.run_step();

        let (manfred, velocity) = world.get_manfred_entity();
        assert_eq!(0, velocity.x());
        assert!(velocity.y() > velocity_after_step_1.y());
        assert_eq!(Direction::Up, manfred.view_direction);
    }

    #[test]
    fn when_w_and_then_d_pressed_then_accelerates_diagonally() {
        let mut world = WorldWrapper::init();

        world.given_key_pressed(KeyCode::W);
        world.given_key_pressed(KeyCode::D);

        world.run_step();

        let (manfred, velocity) = world.get_manfred_entity();
        let velocity_after_step_1 = velocity.clone();
        assert!(velocity_after_step_1.x() > 0);
        assert!(velocity_after_step_1.y() > 0);
        assert_eq!(Direction::Up, manfred.view_direction);

        world.run_step();

        let (manfred, velocity) = world.get_manfred_entity();
        assert!(velocity.x() > velocity_after_step_1.x());
        assert!(velocity.y() > velocity_after_step_1.y());
        assert_eq!(Direction::Up, manfred.view_direction);
    }

    #[test]
    fn when_opposite_directions_pressed_then_does_not_move() {
        let mut world = WorldWrapper::init();

        world.given_key_pressed(KeyCode::A);
        world.given_key_pressed(KeyCode::D);

        world.run_step();

        let (manfred, velocity) = world.get_manfred_entity();
        assert!(!velocity.is_moving());
        assert_eq!(Direction::Down, manfred.view_direction);
    }

    #[test]
    fn when_button_is_released_then_stops_moving_but_keeps_view_direction() {
        let mut world = WorldWrapper::init();

        world.given_key_pressed(KeyCode::D);

        world.run_step();

        let (manfred, velocity) = world.get_manfred_entity();
        assert!(velocity.is_moving());
        assert_eq!(Direction::Right, manfred.view_direction);

        world.given_key_released(KeyCode::D);

        world.run_step();

        let (manfred, velocity) = world.get_manfred_entity();
        assert!(!velocity.is_moving());
        assert_eq!(Direction::Right, manfred.view_direction);
    }

    struct WorldWrapper {
        manfred_id: Entity,
        world: World,
        system_stage: SystemStage,
    }

    impl WorldWrapper {
        fn init() -> WorldWrapper {
            let mut world = World::default();

            let mut system_stage = SystemStage::parallel();
            system_stage.add_system(velocity_control_system.system());

            world.insert_resource(Input::<KeyCode>::default());

            let manfred_id = world
                .spawn()
                .insert(Manfred::default())
                .insert(Velocity::new(5))
                .id();

            WorldWrapper {
                manfred_id,
                world,
                system_stage,
            }
        }

        fn run_step(&mut self) {
            self.system_stage.run(&mut self.world);
        }

        fn get_manfred_entity(&self) -> (&Manfred, &Velocity) {
            let manfred = self.world.get::<Manfred>(self.manfred_id).unwrap();
            let velocity = self.world.get::<Velocity>(self.manfred_id).unwrap();
            (manfred, velocity)
        }

        fn given_key_pressed(&mut self, key: KeyCode) {
            let mut input_resource = self.world.get_resource_mut::<Input<KeyCode>>().unwrap();
            input_resource.press(key);
        }

        fn given_key_released(&mut self, key: KeyCode) {
            let mut input_resource = self.world.get_resource_mut::<Input<KeyCode>>().unwrap();
            input_resource.update();
            input_resource.release(key);
        }
    }
}
