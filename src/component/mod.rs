use std::cmp::Ordering;

use bevy::prelude::IVec2;

pub mod manfred;

pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Velocity {
    x: i32,
    y: i32,
}

impl Velocity {
    pub fn new() -> Velocity {
        Velocity { x: 0, y: 0 }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn accelerate(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }

        println!("after accelerating in {:?}: {:?}", direction, self,);
    }

    pub fn is_moving(&self) -> bool {
        self.x != 0 || self.y != 0
    }

    pub fn get_direction(&self) -> Direction {
        match Ord::cmp(&(-self.y), &self.x.abs()) {
            Ordering::Greater => return Direction::Down,
            Ordering::Equal if self.x <= 0 => return Direction::Down,
            _ => {}
        }

        match Ord::cmp(&self.x, &self.y.abs()) {
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

#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use crate::{Direction, Velocity};

    #[rstest]
    #[case(0, 0, Direction::Down)]
    #[case(2, 1, Direction::Right)]
    #[case(2, 2, Direction::Up)]
    #[case(1, 2, Direction::Up)]
    #[case(-1, 2, Direction::Up)]
    #[case(-2, 2, Direction::Left)]
    #[case(-2, 1, Direction::Left)]
    #[case(-2, -1, Direction::Left)]
    #[case(-2, -2, Direction::Down)]
    #[case(-1, -2, Direction::Down)]
    #[case(1, -2, Direction::Down)]
    #[case(2, -2, Direction::Right)]
    #[case(2, -1, Direction::Right)]
    fn direction_of_velocity(
        #[case] x: i32,
        #[case] y: i32,
        #[case] expected_direction: Direction,
    ) {
        let under_test = Velocity { x, y };

        assert_eq!(
            expected_direction,
            under_test.get_direction(),
            "values were x: {}, y: {}",
            x,
            y
        );
    }
}
