use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::{Cell, Checker, Turn};
use crate::utility::Point;

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
        let cell = self.get_cell(point);
        match (cell, point.y) {
            (Cell::Checker(Checker::White), 0) => self.set_cell(point, cell.promote()),
            (Cell::Checker(Checker::Black), 7) => self.set_cell(point, cell.promote()),
            _ => {}
        };
    }

    pub fn set_cell(&mut self, point: Point, cell: Cell) {
        self.cells[point.y as usize][point.x as usize] = cell;
    }

    pub fn get_cell(&self, point: Point) -> &Cell {
        &self.cells[point.y as usize][point.x as usize]
    }

    pub fn clear_moves(&self) -> Board {
        self.iter().filter(|(.., cell)| cell.is_move()).fold(
            self.clone(),
            |mut board, (point, ..)| {
                board.set_cell(point, Cell::Empty);
                board
            },
        )
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
