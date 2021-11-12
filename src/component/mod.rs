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

pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

impl Velocity {
    pub fn new() -> Velocity {
        Velocity { x: 0, y: 0 }
    }
}
