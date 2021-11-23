use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct LimitedInt<const LIMIT: u16> {
    value: i32,
}

impl<const LIMIT: u16> LimitedInt<LIMIT> {
    pub fn new(mut value: i32) -> LimitedInt<LIMIT> {
        if value > LIMIT as i32 {
            value = LIMIT as i32;
        } else if value < -(LIMIT as i32) {
            value = -(LIMIT as i32);
        };
        LimitedInt { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn abs(&self) -> i32 {
        match self.value.cmp(&0) {
            Ordering::Less => -self.value,
            Ordering::Equal | Ordering::Greater => self.value,
        }
    }
}

impl<const LIMIT: u16> Add<i32> for LimitedInt<LIMIT> {
    type Output = Self;

    fn add(mut self, rhs: i32) -> Self::Output {
        self += rhs;
        self
    }
}

impl<const LIMIT: u16> AddAssign<i32> for LimitedInt<LIMIT> {
    fn add_assign(&mut self, rhs: i32) {
        let new_value = &self.value + rhs;
        self.value = if new_value <= LIMIT as i32 {
            new_value
        } else {
            LIMIT as i32
        };
    }
}

impl<const LIMIT: u16> Sub<i32> for LimitedInt<LIMIT> {
    type Output = Self;

    fn sub(mut self, rhs: i32) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<const LIMIT: u16> SubAssign<i32> for LimitedInt<LIMIT> {
    fn sub_assign(&mut self, rhs: i32) {
        let new_value = &self.value - rhs;
        self.value = if new_value >= -(LIMIT as i32) {
            new_value
        } else {
            -(LIMIT as i32)
        };
    }
}

impl<const LIMIT: u16> PartialEq<i32> for LimitedInt<LIMIT> {
    fn eq(&self, other: &i32) -> bool {
        &self.value == other
    }
}

impl<const LIMIT: u16> PartialOrd<i32> for LimitedInt<LIMIT> {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        Some(self.value.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use crate::types::limited_int::LimitedInt;

    #[rstest]
    #[case(0, 0)]
    #[case(2, 2)]
    #[case(-3, -3)]
    #[case(7, 5)]
    #[case(-10, -5)]
    fn initialization_does_not_exceed_limits(#[case] initial_value: i32, #[case] expected: i32) {
        let under_test = LimitedInt::<5>::new(initial_value);

        assert_eq!(expected, under_test.value());
    }

    #[test]
    fn addition_within_limit() {
        let mut under_test = LimitedInt::<5>::new(0);

        under_test = under_test + 1;
        assert_eq!(1, under_test.value());

        under_test = under_test + 2;
        assert_eq!(3, under_test.value());

        under_test = under_test + 2;
        assert_eq!(5, under_test.value());
    }

    #[test]
    fn subtraction_within_limit() {
        let mut under_test = LimitedInt::<5>::new(0);

        under_test = under_test - 1;
        assert_eq!(-1, under_test.value());

        under_test = under_test - 2;
        assert_eq!(-3, under_test.value());

        under_test = under_test - 2;
        assert_eq!(-5, under_test.value());
    }

    #[rstest]
    #[case(3, 4)]
    #[case(2, 2)]
    #[case(0, 4)]
    #[case(0, 7)]
    #[case(-2, 15)]
    fn addition_exceeding_limit(#[case] initial_value: i32, #[case] summand: i32) {
        let mut under_test = LimitedInt::<3>::new(initial_value);

        under_test = under_test + summand;

        assert_eq!(3, under_test.value())
    }

    #[rstest]
    #[case(-3, 4)]
    #[case(-2, 2)]
    #[case(0, 4)]
    #[case(0, 7)]
    #[case(2, 15)]
    fn subtraction_exceeding_limit(#[case] initial_value: i32, #[case] subtrahend: i32) {
        let mut under_test = LimitedInt::<3>::new(initial_value);

        under_test = under_test - subtrahend;

        assert_eq!(-3, under_test.value())
    }
}
