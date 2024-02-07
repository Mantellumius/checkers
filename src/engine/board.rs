use serde::{Deserialize, Serialize};
use std::{fmt::Display, vec};

use super::{Cell, Checker, Route, Turn};
use crate::utility::{Point, Shift};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Board {
    pub cells: [[Cell; 8]; 8],
    pub turn: Turn,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self::default();
        for i in 0..8 {
            for j in 0..8 {
                if (i + j) % 2 != 0 {
                    match i {
                        0..=2 => board.cells[i][j] = Cell::Checker(Checker::Black),
                        5..=7 => board.cells[i][j] = Cell::Checker(Checker::White),
                        _ => continue,
                    }
                }
            }
        }
        board
    }

    pub fn iter(&self) -> BoardIterator {
        BoardIterator {
            board: self,
            x: 0,
            y: 0,
        }
    }

    pub fn check_promotion(&mut self, point: Point) {
        let cell = *self.get_cell(point);
        if !cell.is_queen() && (point.y == 0 || point.y == 7) {
            self.set_cell(point, cell.promote());
        }
    }

    

    pub fn set_cell(&mut self, point: Point, cell: Cell) {
        self.cells[point.y as usize][point.x as usize] = cell;
    }

    pub fn get_cell(&self, point: Point) -> &Cell {
        &self.cells[point.y as usize][point.x as usize]
    }

    pub fn get_enemy_neighbours(&self, point: Point) -> Vec<Point> {
        let cell = self.get_cell(point);
        self.get_neighbours(point)
            .into_iter()
            .filter(|neighbour_point| self.get_cell(*neighbour_point).is_enemy(cell))
            .collect()
    }

    pub fn get_enemy_neighbours_queen(&self, point: Point) -> Vec<Point> {
        let cell = self.get_cell(point);
        self.get_neighbours_queen(point)
            .into_iter()
            .filter(|neighbour_point| self.get_cell(*neighbour_point).is_enemy(cell))
            .collect()
    }

    pub fn get_neighbours(&self, point: Point) -> Vec<Point> {
        let deltas = vec![
            Point::new(-1, -1),
            Point::new(-1, 1),
            Point::new(1, -1),
            Point::new(1, 1),
        ];
        deltas
            .into_iter()
            .map(|delta| delta.add(&point))
            .filter(|point| point.valid())
            .collect()
    }

    pub fn get_neighbours_queen(&self, point: Point) -> Vec<Point> {
        let mut result = vec![];
        let mut points = [point, point, point, point];
        let mut finished = [false, false, false, false];
        let deltas = [
            Point::new(-1, -1),
            Point::new(-1, 1),
            Point::new(1, -1),
            Point::new(1, 1),
        ];
        while points.iter().any(|delta| delta.valid()) {
            for i in 0..=3 {
                points[i] = points[i].add(&deltas[i]);
                if finished[i] || !points[i].valid() {
                    continue;
                }
                if self.get_cell(points[i]).is_empty() {
                    result.push(points[i]);
                } else if self.get_cell(points[i]).is_checker() {
                    result.push(points[i]);
                    finished[i] = true;
                } else {
                    finished[i] = true;
                }
            }
        }
        result
    }

    pub fn clear_moves(&self) -> Board {
        let mut board = self.clone();
        self.iter()
            .filter(|(.., cell)| cell.is_move())
            .for_each(|(point, ..)| board.set_cell(point, Cell::Empty));
        board
    }
}

pub struct BoardIterator<'a> {
    board: &'a Board,
    x: i8,
    y: i8,
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = (Point, &'a Cell);

    fn next(&mut self) -> Option<Self::Item> {
        self.x += 1;
        if self.x == 8 {
            self.x = 0;
            self.y += 1;
        }
        if self.y == 8 {
            return None;
        }
        let point = Point {
            x: self.x,
            y: self.y,
        };
        Some((point, self.board.get_cell(point)))
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for checker in row {
                write!(f, "{}", checker)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
