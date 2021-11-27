pub mod limited_int;

#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
