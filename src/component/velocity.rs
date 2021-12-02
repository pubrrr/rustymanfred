use std::cmp::{max, Ordering};

use crate::types::limited_int::LimitedInt;
use crate::types::Direction;

#[derive(Debug, Clone)]
pub struct Velocity<const ACCELERATION_STEPS: u16> {
    x: LimitedInt<ACCELERATION_STEPS>,
    y: LimitedInt<ACCELERATION_STEPS>,
    max_speed: i32,
}

impl<const ACCELERATION_STEPS: u16> Velocity<ACCELERATION_STEPS> {
    pub fn new(max_speed: u16) -> Velocity<ACCELERATION_STEPS> {
        Velocity {
            x: LimitedInt::new(0),
            y: LimitedInt::new(0),
            max_speed: max_speed as i32,
        }
    }

    pub fn x(&self) -> i32 {
        match self.y.value() {
            0 => self.scale_coordinate_by_acceleration_steps(self.x.value()),
            y => self.scale_coordinate_by_length(self.x.value(), y),
        }
    }

    pub fn y(&self) -> i32 {
        match self.x.value() {
            0 => self.scale_coordinate_by_acceleration_steps(self.y.value()),
            x => self.scale_coordinate_by_length(self.y.value(), x),
        }
    }

    fn scale_coordinate_by_acceleration_steps(&self, coordinate: i32) -> i32 {
        let acceleration_steps = ACCELERATION_STEPS as i32;

        if coordinate == acceleration_steps {
            self.max_speed
        } else if coordinate == -acceleration_steps {
            -self.max_speed
        } else {
            coordinate * self.max_speed / acceleration_steps
        }
    }

    fn scale_coordinate_by_length(&self, coord: i32, other_coord: i32) -> i32 {
        let actual_acceleration_step = max(coord.abs(), other_coord.abs());
        let current_speed = self.max_speed * actual_acceleration_step / ACCELERATION_STEPS as i32;

        let length = ((coord * coord + other_coord * other_coord) as f32).sqrt();

        ((coord * current_speed) as f32 / length).round() as i32
    }

    pub fn accelerate(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    pub fn decelerate(&mut self, direction: Direction) {
        match direction {
            Direction::Up if self.y > 0 => self.y -= 1,
            Direction::Down if self.y < 0 => self.y += 1,
            Direction::Left if self.x < 0 => self.x += 1,
            Direction::Right if self.x > 0 => self.x -= 1,
            _ => {}
        }
    }

    pub fn is_moving(&self) -> bool {
        self.x != 0 || self.y != 0
    }

    pub fn get_direction(&self) -> Direction {
        match Ord::cmp(&(-self.y.value()), &self.x.abs()) {
            Ordering::Greater => return Direction::Down,
            Ordering::Equal if self.x <= 0 => return Direction::Down,
            _ => {}
        }

        match Ord::cmp(&self.x.value(), &self.y.abs()) {
            Ordering::Greater => return Direction::Right,
            Ordering::Equal if self.y < 0 => return Direction::Right,
            _ => {}
        }

        if self.x <= -self.y.abs() {
            return Direction::Left;
        }

        Direction::Up
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use crate::component::velocity::Velocity;
    use crate::types::Direction;

    #[rstest]
    #[case::no_acceleration(vec![], Direction::Down)]
    #[case::down(vec![Direction::Down], Direction::Down)]
    #[case::down_right(vec![Direction::Down, Direction::Right], Direction::Right)]
    #[case::right(vec![Direction::Right], Direction::Right)]
    #[case::up_right(vec![Direction::Up, Direction::Right], Direction::Up)]
    #[case::up(vec![Direction::Up], Direction::Up)]
    #[case::up_left(vec![Direction::Up, Direction::Left], Direction::Left)]
    #[case::left(vec![Direction::Left], Direction::Left)]
    #[case::down_left(vec![Direction::Down, Direction::Left], Direction::Down)]
    fn direction_of_velocity(
        #[case] acceleration_steps: Vec<Direction>,
        #[case] expected_direction: Direction,
    ) {
        let mut under_test = Velocity::<5>::new(10);

        for acceleration in acceleration_steps {
            under_test.accelerate(acceleration);
        }

        assert_eq!(expected_direction, under_test.get_direction());
    }

    #[rstest]
    #[case::down_left(vec![Direction::Down, Direction::Left], vec![Direction::Left, Direction::Down])]
    #[case::down_right(vec![Direction::Down, Direction::Right], vec![Direction::Right, Direction::Down])]
    #[case::up_left(vec![Direction::Up, Direction::Left], vec![Direction::Left, Direction::Up])]
    #[case::up_right(vec![Direction::Up, Direction::Right], vec![Direction::Right, Direction::Up])]
    #[case::many_ups_and_right(vec![Direction::Up, Direction::Up, Direction::Up, Direction::Up, Direction::Right], vec![Direction::Up, Direction::Right, Direction::Up, Direction::Up, Direction::Up])]
    fn acceleration_is_commutative(
        #[case] acceleration_steps: Vec<Direction>,
        #[case] commuted_acceleration_steps: Vec<Direction>,
    ) {
        let mut under_test = Velocity::<5>::new(10);

        for acceleration in acceleration_steps {
            under_test.accelerate(acceleration);
        }

        let mut comparison_velocity = Velocity::<5>::new(10);

        for acceleration in commuted_acceleration_steps {
            comparison_velocity.accelerate(acceleration);
        }

        assert_eq!(comparison_velocity.x(), under_test.x());
        assert_eq!(comparison_velocity.y(), under_test.y());
        assert_eq!(
            comparison_velocity.get_direction(),
            under_test.get_direction()
        );
    }

    #[test]
    fn accelerate_in_different_direction() {
        let mut under_test = Velocity::<5>::new(5);

        under_test.accelerate(Direction::Left);
        assert_eq!(-1, under_test.x());
        assert_eq!(0, under_test.y());

        under_test.accelerate(Direction::Down);
        assert_eq!(-1, under_test.x());
        assert_eq!(-1, under_test.y());

        under_test.accelerate(Direction::Right);
        assert_eq!(0, under_test.x());
        assert_eq!(-1, under_test.y());

        under_test.accelerate(Direction::Up);
        assert_eq!(0, under_test.x());
        assert_eq!(0, under_test.y());
    }

    #[rstest]
    #[case::down(Direction::Down)]
    #[case::right(Direction::Right)]
    #[case::up(Direction::Up)]
    #[case::left(Direction::Left)]
    fn accelerate_and_decelerate(#[case] direction: Direction) {
        let mut under_test = Velocity::<5>::new(10);

        under_test.accelerate(direction);
        assert!(under_test.is_moving());

        under_test.accelerate(direction);
        assert!(under_test.is_moving());

        under_test.decelerate(direction);
        assert!(under_test.is_moving());

        under_test.decelerate(direction);
        assert!(!under_test.is_moving());

        under_test.decelerate(direction);
        assert!(!under_test.is_moving());
    }

    #[test]
    fn is_not_moving_initally() {
        let under_test = Velocity::<5>::new(5);

        assert!(!under_test.is_moving());
    }

    #[test]
    fn x_coordinate_scales_by_speed() {
        let mut under_test = Velocity::<2>::new(40);

        under_test.accelerate(Direction::Right);
        assert_eq!(20, under_test.x());
        assert_eq!(0, under_test.y());

        under_test.accelerate(Direction::Right);
        assert_eq!(40, under_test.x());
        assert_eq!(0, under_test.y());

        under_test.accelerate(Direction::Right);
        assert_eq!(40, under_test.x());
        assert_eq!(0, under_test.y());
    }

    #[test]
    fn y_coordinate_scales_by_speed() {
        let mut under_test = Velocity::<2>::new(40);

        under_test.accelerate(Direction::Up);
        assert_eq!(0, under_test.x());
        assert_eq!(20, under_test.y());

        under_test.accelerate(Direction::Up);
        assert_eq!(0, under_test.x());
        assert_eq!(40, under_test.y());

        under_test.accelerate(Direction::Up);
        assert_eq!(0, under_test.x());
        assert_eq!(40, under_test.y());
    }

    #[test]
    fn x_coordinate_scales_by_speed_negative_direction() {
        let mut under_test = Velocity::<2>::new(40);

        under_test.accelerate(Direction::Left);
        assert_eq!(-20, under_test.x());
        assert_eq!(0, under_test.y());

        under_test.accelerate(Direction::Left);
        assert_eq!(-40, under_test.x());
        assert_eq!(0, under_test.y());

        under_test.accelerate(Direction::Left);
        assert_eq!(-40, under_test.x());
        assert_eq!(0, under_test.y());
    }

    #[test]
    fn y_coordinate_scales_by_speed_negative_direction() {
        let mut under_test = Velocity::<2>::new(40);

        under_test.accelerate(Direction::Down);
        assert_eq!(0, under_test.x());
        assert_eq!(-20, under_test.y());

        under_test.accelerate(Direction::Down);
        assert_eq!(0, under_test.x());
        assert_eq!(-40, under_test.y());

        under_test.accelerate(Direction::Down);
        assert_eq!(0, under_test.x());
        assert_eq!(-40, under_test.y());
    }

    #[test]
    fn when_moving_diagonally_then_speed_is_not_greater_than_max_speed() {
        let mut under_test = Velocity::<4>::new(120);

        under_test.accelerate(Direction::Right);
        under_test.accelerate(Direction::Right);
        under_test.accelerate(Direction::Right);
        under_test.accelerate(Direction::Right);
        assert_eq!(120, under_test.x());
        assert_eq!(0, under_test.y());

        under_test.accelerate(Direction::Down);
        under_test.accelerate(Direction::Down);
        under_test.accelerate(Direction::Down);
        assert_eq!(96, under_test.x());
        assert_eq!(-72, under_test.y());

        under_test.accelerate(Direction::Down);
        assert_eq!(85, under_test.x());
        assert_eq!(-85, under_test.y());
    }

    #[test]
    fn given_moving_right_when_accelerating_upwards_then_speed_remains_constant_unless_y_exceeds_x()
    {
        let mut under_test = Velocity::<5>::new(1500);

        under_test.accelerate(Direction::Right);
        under_test.accelerate(Direction::Right);
        under_test.accelerate(Direction::Right);
        under_test.accelerate(Direction::Right);
        let x = under_test.x();
        let y = under_test.y();
        assert_eq!(1440000, x * x + y * y);

        under_test.accelerate(Direction::Up);
        let x = under_test.x();
        let y = under_test.y();
        assert!(1441000 > x * x + y * y, "x*x+y*y: {}", x * x + y * y);
        assert!(1439000 < x * x + y * y, "x*x+y*y: {}", x * x + y * y);

        under_test.accelerate(Direction::Up);
        let x = under_test.x();
        let y = under_test.y();
        assert!(1441000 > x * x + y * y, "x*x+y*y: {}", x * x + y * y);
        assert!(1439000 < x * x + y * y, "x*x+y*y: {}", x * x + y * y);

        under_test.accelerate(Direction::Up);
        let x = under_test.x();
        let y = under_test.y();
        assert!(1441000 > x * x + y * y, "x*x+y*y: {}", x * x + y * y);
        assert!(1439000 < x * x + y * y, "x*x+y*y: {}", x * x + y * y);

        under_test.accelerate(Direction::Up);
        let x = under_test.x();
        let y = under_test.y();
        assert!(1442000 > x * x + y * y, "x*x+y*y: {}", x * x + y * y);
        assert!(1439000 < x * x + y * y, "x*x+y*y: {}", x * x + y * y);

        under_test.accelerate(Direction::Up);
        let x = under_test.x();
        let y = under_test.y();
        assert!(2251000 > x * x + y * y, "x*x+y*y: {}", x * x + y * y);
        assert!(2249000 < x * x + y * y, "x*x+y*y: {}", x * x + y * y);
    }
}
