use std::ops::{Add, AddAssign};

use super::{CardinalDirection, Direction};

/// Represent a 2D coordinate.
#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    /// Create a new coordinate.
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Direction) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Direction> for Coordinate {
    fn add_assign(&mut self, rhs: Direction) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<CardinalDirection> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: CardinalDirection) -> Self::Output {
        let dir = Direction::from(rhs);
        Coordinate {
            x: self.x + dir.x,
            y: self.y + dir.y,
        }
    }
}

impl AddAssign<CardinalDirection> for Coordinate {
    fn add_assign(&mut self, rhs: CardinalDirection) {
        let dir = Direction::from(rhs);
        self.x += dir.x;
        self.y += dir.y;
    }
}

impl From<Coordinate> for (i32, i32) {
    fn from(coordinate: Coordinate) -> Self {
        (coordinate.x, coordinate.y)
    }
}
