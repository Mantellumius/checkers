use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Default)]
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

    pub fn divide(&self, divider: i8) -> Point {
        Point::new(self.x / divider, self.y / divider)
    }

    pub fn valid(&self) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < 8 && self.y < 8
    }

    pub fn signum(&self) -> Point {
        Point::new(self.x.signum(), self.y.signum())
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub trait Shift<T> {
    fn shift(&mut self) -> Option<T>;
}

impl<T> Shift<T> for Vec<T> {
    fn shift(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        Some(self.remove(0))
    }
}

pub trait FindAndRemove<T> {
    fn find_and_remove<P>(&mut self, predicate: P) -> Option<T>
    where
        P: FnMut(&T) -> bool;
}

impl<T> FindAndRemove<T> for Vec<T> {
    fn find_and_remove<P>(&mut self, predicate: P) -> Option<T>
    where
        P: FnMut(&T) -> bool,
    {
        match self.iter().position(predicate) {
            Some(value_index) => {
                let route = self.remove(value_index);
                Some(route)
            }
            None => None,
        }
    }
}

pub trait FindAndRemoveAll<T> {
    fn find_and_remove_all<P>(&mut self, predicate: P) -> Vec<T>
    where
        P: FnMut(&T) -> bool;
}

impl<T> FindAndRemoveAll<T> for Vec<T> {
    fn find_and_remove_all<P>(&mut self, mut predicate: P) -> Vec<T>
    where
        P: FnMut(&T) -> bool,
    {
        let mut result = Vec::new();
        while let Some(value) = self.find_and_remove(&mut predicate) {
            result.push(value);
        }
        result
    }
}
