use std::cmp::Ordering;

use crate::types::limited_int::LimitedInt;
use crate::types::Direction;

#[derive(Debug, Clone)]
pub struct Velocity {
    x: LimitedInt<10>,
    y: LimitedInt<10>,
    max_speed: u32,
}

impl Velocity {
    pub fn new(max_speed: u32) -> Velocity {
        Velocity {
            x: LimitedInt::new(0),
            y: LimitedInt::new(0),
            max_speed,
        }
    }

    pub fn x(&self) -> i32 {
        self.x.value()
    }

    pub fn y(&self) -> i32 {
        self.y.value()
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

    use crate::types::Direction;
    use crate::Velocity;

    #[rstest]
    #[case(vec![], Direction::Down)]
    #[case(vec![Direction::Down], Direction::Down)]
    #[case(vec![Direction::Down, Direction::Right], Direction::Right)]
    #[case(vec![Direction::Right], Direction::Right)]
    #[case(vec![Direction::Up, Direction::Right], Direction::Up)]
    #[case(vec![Direction::Up], Direction::Up)]
    #[case(vec![Direction::Up, Direction::Left], Direction::Left)]
    #[case(vec![Direction::Left], Direction::Left)]
    #[case(vec![Direction::Down, Direction::Left], Direction::Down)]
    fn direction_of_velocity(
        #[case] acceleration_steps: Vec<Direction>,
        #[case] expected_direction: Direction,
    ) {
        let mut under_test = Velocity::new(10);

        for acceleration in acceleration_steps {
            under_test.accelerate(acceleration);
        }

        assert_eq!(expected_direction, under_test.get_direction());
    }

    #[rstest]
    #[case(vec![Direction::Down, Direction::Left], vec![Direction::Left, Direction::Down])]
    #[case(vec![Direction::Down, Direction::Right], vec![Direction::Right, Direction::Down])]
    #[case(vec![Direction::Up, Direction::Left], vec![Direction::Left, Direction::Up])]
    #[case(vec![Direction::Up, Direction::Right], vec![Direction::Right, Direction::Up])]
    #[case(vec![Direction::Up, Direction::Up, Direction::Up, Direction::Up, Direction::Right], vec![Direction::Up, Direction::Right, Direction::Up, Direction::Up, Direction::Up])]
    fn acceleration_is_commutative(
        #[case] acceleration_steps: Vec<Direction>,
        #[case] commuted_acceleration_steps: Vec<Direction>,
    ) {
        let mut under_test = Velocity::new(10);

        for acceleration in acceleration_steps {
            under_test.accelerate(acceleration);
        }

        let mut comparison_velocity = Velocity::new(10);

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
        let mut under_test = Velocity::new(10);

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
    #[case(Direction::Down)]
    #[case(Direction::Right)]
    #[case(Direction::Up)]
    #[case(Direction::Left)]
    fn accelerate_and_decelerate(#[case] direction: Direction) {
        let mut under_test = Velocity::new(10);

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
        let under_test = Velocity::new(5);

        assert!(!under_test.is_moving());
    }
}
