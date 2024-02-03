use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::{Cell, Checker};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Board {
    pub cells: [[Cell; 8]; 8],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [[Cell::Empty; 8]; 8],
        }
    }
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

    fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y][x]
    }

    fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[y][x] = cell;
    }

    fn get_empty_neighbour_cells(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        self.get_neighbour_cells(x, y)
            .into_iter()
            .filter(|n| self.get_cell(n.0, n.1).is_empty())
            .collect()
    }

    fn get_neighbour_cells(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours = vec![];
        if y > 0 && x > 0 {
            neighbours.push((x - 1, y - 1));
        }
        if y < 7 && x > 0 {
            neighbours.push((x - 1, y + 1));
        }
        if y > 0 && x < 7 {
            neighbours.push((x + 1, y - 1));
        }
        if y < 7 && x < 7 {
            neighbours.push((x + 1, y + 1));
        }
        neighbours
    }

    pub fn with_legal_moves(&self, x: usize, y: usize) -> Self {
        let mut new_board = self.clone();
        let neighbours = new_board.get_empty_neighbour_cells(x, y);
        neighbours
            .into_iter()
            .for_each(|(x, y)| new_board.set_cell(x, y, Cell::Move));
        new_board
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
