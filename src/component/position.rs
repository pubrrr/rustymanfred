#[derive(Debug, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::new(0, 0)
    }
}
