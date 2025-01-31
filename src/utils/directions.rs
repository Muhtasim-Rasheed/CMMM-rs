use std::fmt::{Debug, Formatter, Result};

pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}

impl Clone for Directions {
    fn clone(&self) -> Directions {
        match self {
            Directions::Up => Directions::Up,
            Directions::Down => Directions::Down,
            Directions::Left => Directions::Left,
            Directions::Right => Directions::Right,
        }
    }
}

impl Debug for Directions {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Directions::Up => write!(f, "Up"),
            Directions::Down => write!(f, "Down"),
            Directions::Left => write!(f, "Left"),
            Directions::Right => write!(f, "Right"),
        }
    }
}
