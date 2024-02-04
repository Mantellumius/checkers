use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}

impl Point {
    pub fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }

    pub fn subtract(&self, other: &Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }

    pub fn add(&self, other: &Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }

    pub fn valid(&self) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < 8 && self.y < 8
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
