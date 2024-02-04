use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::utility::Point;

use super::{Cell, Checker, Turn};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Board {
    pub cells: [[Cell; 8]; 8],
    pub turn: Turn,
    pub selected_cell: Option<Point>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [[Cell::Empty; 8]; 8],
            turn: Turn::White,
            selected_cell: None,
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

    pub fn is_selected(&self, x: &usize, y: &usize) -> bool {
        self.selected_cell
            .map_or(false, |selected| selected.x == *x && selected.y == *y)
    }

    pub fn iter(&self) -> BoardIterator {
        BoardIterator {
            board: self,
            x: 0,
            y: 0,
        }
    }

    pub fn with_legal_moves(&self, point: Point) -> Self {
        let mut new_board = self.clear_moves();
        let neighbours = new_board.get_empty_neighbours(point);
        let cell = *new_board.get_cell(point);
        neighbours
            .into_iter()
            .filter(|neighbor_point| match cell {
                Cell::Checker(Checker::White) => neighbor_point.y < point.y,
                Cell::Checker(Checker::Black) => neighbor_point.y > point.y,
                Cell::Checker(Checker::BlackQueen) => true,
                Cell::Checker(Checker::WhiteQueen) => true,
                _ => false,
            })
            .for_each(|point| new_board.set_cell(point, Cell::Move));
        new_board.selected_cell = Some(point);
        new_board
    }

    pub fn make_move(&self, target: Point) -> Self {
        let mut new_board = self.clear_moves();
        match self.selected_cell {
            None => new_board,
            Some(point) => {
                let cell = *new_board.get_cell(point);
                new_board.set_cell(point, Cell::Empty);
                new_board.set_cell(target, cell);
                new_board.selected_cell = None;
                new_board.turn = new_board.turn.next();
                new_board
            }
        }
    }

    fn set_cell(&mut self, target: Point, cell: Cell) {
        self.cells[target.y][target.x] = cell;
    }

    fn get_cell(&self, point: Point) -> &Cell {
        &self.cells[point.y][point.x]
    }

    fn get_empty_neighbours(&self, point: Point) -> Vec<Point> {
        self.get_neighbours(point)
            .filter(|point| self.get_cell(*point).is_empty())
            .collect()
    }

    fn clear_moves(&self) -> Board {
        let mut new_board = self.clone();
        self.iter()
            .filter(|(.., cell)| cell.is_move())
            .for_each(|(point, ..)| {
                new_board.set_cell(point, Cell::Empty);
            });
        new_board
    }

    fn get_neighbours(&self, point: Point) -> impl Iterator<Item = Point> {
        let mut neighbours = vec![];
        let Point { x, y } = point;
        if y > 0 && x > 0 {
            neighbours.push(Point { x: x - 1, y: y - 1 });
        }
        if y < 7 && x > 0 {
            neighbours.push(Point { x: x - 1, y: y + 1 });
        }
        if y > 0 && x < 7 {
            neighbours.push(Point { x: x + 1, y: y - 1 });
        }
        if y < 7 && x < 7 {
            neighbours.push(Point { x: x + 1, y: y + 1 });
        }
        neighbours.into_iter()
    }
}

pub struct BoardIterator<'a> {
    board: &'a Board,
    x: usize,
    y: usize,
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
