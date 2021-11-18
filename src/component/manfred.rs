use crate::Direction;

pub struct Manfred {
    pub view_direction: Direction,
}

impl Default for Manfred {
    fn default() -> Self {
        Manfred {
            view_direction: Direction::Down,
        }
    }
}
