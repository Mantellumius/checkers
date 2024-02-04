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
        self.selected_cell.map_or(false, |selected| {
            selected.x == *x as i8 && selected.y == *y as i8
        })
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
        let empty_neighbors = new_board.get_empty_neighbours(point);
        let captures = new_board.get_captures(point);
        let cell = *new_board.get_cell(point);
        empty_neighbors
            .into_iter()
            .filter(|neighbor_point| match cell {
                Cell::Checker(Checker::White) => neighbor_point.y < point.y,
                Cell::Checker(Checker::Black) => neighbor_point.y > point.y,
                Cell::Checker(Checker::BlackQueen) => true,
                Cell::Checker(Checker::WhiteQueen) => true,
                _ => false,
            })
            .for_each(|point| new_board.set_cell(point, Cell::Move));
        captures
            .into_iter()
            .for_each(|point| new_board.set_cell(point, Cell::Capture));
        new_board.selected_cell = Some(point);
        new_board
    }

    pub fn with_captures(&self, start: Point) -> Self {
        let mut new_board = self.clear_moves();
        new_board.selected_cell = Some(start);
        let captures = self.get_captures(start);
        captures
            .into_iter()
            .for_each(|point| new_board.set_cell(point, Cell::Move));
        new_board
    }

    pub fn make_move(&self, target: Point) -> Self {
        let mut new_board = self.clear_moves();
        let target_cell = *new_board.get_cell(target);
        if let Some(point) = self.selected_cell {
            let cell = *new_board.get_cell(point);
            match target_cell {
                Cell::Move => {
                    new_board.set_cell(point, Cell::Empty);
                    new_board.set_cell(target, cell);
                }
                Cell::Capture => {
                    new_board.set_cell(point, Cell::Empty);
                    new_board.set_cell(target, cell);
                }
                _ => {}
            }
            new_board.turn = new_board.turn.next();
            new_board.selected_cell = None;
        }
        new_board
    }

    fn get_captures(&self, start: Point) -> Vec<Point> {
        let mut new_board = self.clear_moves();
        new_board.selected_cell = Some(start);
        let mut result = vec![];
        let mut captures = vec![start];
        while let Some(capture_point) = captures.pop() {
            let enemy_neighbors = new_board
                .make_move(capture_point)
                .get_enemy_neighbours(capture_point);
            enemy_neighbors.into_iter().for_each(|neighbour_point| {
                let delta = neighbour_point.subtract(&capture_point);
                let point_behind_enemy = neighbour_point.add(&delta);
                if point_behind_enemy.valid() && new_board.get_cell(point_behind_enemy).is_empty() {
                    new_board.set_cell(point_behind_enemy, Cell::Move);
                    captures.push(point_behind_enemy);
                    result.push(point_behind_enemy);
                }
            });
        }
        result
    }

    fn set_cell(&mut self, target: Point, cell: Cell) {
        self.cells[target.y as usize][target.x as usize] = cell;
    }

    fn get_cell(&self, point: Point) -> &Cell {
        &self.cells[point.y as usize][point.x as usize]
    }

    fn get_enemy_neighbours(&self, point: Point) -> Vec<Point> {
        let cell = self.get_cell(point);
        self.get_neighbours(point)
            .filter(|neighbour_point| self.get_cell(*neighbour_point).is_enemy(*cell))
            .collect()
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
            .for_each(|(point, ..)| new_board.set_cell(point, Cell::Empty));
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
