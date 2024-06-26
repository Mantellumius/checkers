use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::{constants::BOARD_SIZE, Cell, Checker, Turn};
use crate::utility::Point;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Board {
    pub size: usize,
    pub cells: Vec<Vec<Cell>>,
    pub turn: Turn,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self::default();
        let black_last_row = board.size / 2 - 1;
        let white_last_row = board.size / 2;
        for i in 0..board.size {
            for j in 0..board.size {
                if (i + j) % 2 != 0 {
                    match i {
                        _ if i < black_last_row => {
                            board.cells[i][j] = Cell::Checker(Checker::Black)
                        }
                        _ if i > white_last_row => {
                            board.cells[i][j] = Cell::Checker(Checker::White)
                        }
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
        let white_promotion_row = 0;
        let black_promotion_row = (self.size - 1) as i8;
        match cell {
            Cell::Checker(Checker::White) if point.y == white_promotion_row => {
                self.set_cell(point, cell.promote())
            }
            Cell::Checker(Checker::Black) if point.y == black_promotion_row => {
                self.set_cell(point, cell.promote())
            }
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
        if self.x == self.board.size as i8 {
            self.x = 0;
            self.y += 1;
        }
        if self.y == self.board.size as i8 {
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

impl Default for Board {
    fn default() -> Self {
        Self {
            size: BOARD_SIZE,
            cells: vec![vec![Cell::Empty; BOARD_SIZE]; BOARD_SIZE],
            turn: Turn::White,
        }
    }
}
