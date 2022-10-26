use std::ops::{Add, AddAssign};

/// Represent a direction vector in the 2D plane
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

/// Represent a cardinal direction
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl Add for Direction {
    type Output = Direction;

    fn add(self, rhs: Self) -> Self::Output {
        Direction {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Direction {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl From<CardinalDirection> for Direction {
    fn from(card_dir: CardinalDirection) -> Direction {
        match card_dir {
            CardinalDirection::North => Direction { x: 0, y: -1 },
            CardinalDirection::East => Direction { x: 1, y: 0 },
            CardinalDirection::South => Direction { x: 0, y: 1 },
            CardinalDirection::West => Direction { x: -1, y: 0 },
        }
    }
}
