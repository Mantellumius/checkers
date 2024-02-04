use std::fmt::Display;

use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize, 
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
