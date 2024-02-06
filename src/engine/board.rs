use serde::{Deserialize, Serialize};
use std::{fmt::Display, vec};

use super::{Cell, Checker, Route, Turn};
use crate::utility::{Point, Shift};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Board {
    pub cells: [[Cell; 8]; 8],
    pub turn: Turn,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [[Cell::Empty; 8]; 8],
            turn: Turn::White,
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

    pub fn iter(&self) -> BoardIterator {
        BoardIterator {
            board: self,
            x: 0,
            y: 0,
        }
    }

    pub fn with_legal_moves(&self, from: Point) -> Self {
        let mut board = self.clear_moves();
        let cell = *board.get_cell(from);
        if cell.is_queen() {
            let moves: Vec<Point> = board
                .get_neighbours_queen(from)
                .into_iter()
                .filter(|point| board.get_cell(*point).is_empty())
                .filter(|neighbor_point| match cell {
                    Cell::Checker(Checker::White) => neighbor_point.y < from.y,
                    Cell::Checker(Checker::Black) => neighbor_point.y > from.y,
                    Cell::Checker(Checker::BlackQueen) => true,
                    Cell::Checker(Checker::WhiteQueen) => true,
                    _ => false,
                })
                .collect();
            moves
                .into_iter()
                .for_each(|point| board.set_cell(point, Cell::Move));
            board
                .get_captures(from)
                .into_iter()
                .map(|route| route.get_after_last())
                .filter(|route| !route.is_empty())
                .for_each(|route| board.set_cell(*route.last().unwrap(), Cell::Capture));
        } else {
            let moves: Vec<Point> = board
                .get_neighbours(from)
                .into_iter()
                .filter(|point| board.get_cell(*point).is_empty())
                .filter(|neighbor_point| match cell {
                    Cell::Checker(Checker::White) => neighbor_point.y < from.y,
                    Cell::Checker(Checker::Black) => neighbor_point.y > from.y,
                    Cell::Checker(Checker::BlackQueen) => true,
                    Cell::Checker(Checker::WhiteQueen) => true,
                    _ => false,
                })
                .collect();
            moves
                .into_iter()
                .for_each(|point| board.set_cell(point, Cell::Move));
            board
                .get_captures(from)
                .into_iter()
                .map(|route| route.get_after_last())
                .filter(|route| !route.is_empty())
                .for_each(|route| board.set_cell(*route.last().unwrap(), Cell::Capture));
        }
        board
    }

    pub fn make_move(&self, from: Point, to: Point) -> Self {
        let mut board = self.with_legal_moves(from);
        let to_cell = *board.get_cell(to);
        let from_cell = *board.get_cell(from);
        match to_cell {
            Cell::Move => {
                board.set_cell(from, Cell::Empty);
                board.set_cell(to, from_cell);
                board.check_promotion(to);
            }
            Cell::Capture => board = board.capture(from, to),
            _ => {}
        }
        board.turn = board.turn.next();
        board.clear_moves()
    }

    fn capture(&self, from: Point, target: Point) -> Self {
        let mut board = self.clone();
        board
            .get_captures(from)
            .into_iter()
            .find(|route| *route.last().unwrap() == target)
            .unwrap_or_default()
            .iter()
            .reduce(|prev, curr| {
                board = board.simple_capture(*prev, *curr);
                curr
            });
        board
    }

    fn route_capture(&self, route: &Route) -> Self {
        let mut board = self.clone();
        route.iter().reduce(|prev, curr| {
            board = board.simple_capture(*prev, *curr);
            curr
        });
        board
    }

    fn simple_capture(&self, from: Point, target: Point) -> Self {
        let mut board = self.clone();
        let cell = *board.get_cell(from);
        let delta = target.subtract(&from).signum();
        board.set_cell(from, Cell::Empty);
        let mut start = from;
        while start != target {
            start = start.add(&delta);
            board.set_cell(start, Cell::Empty);
        }
        board.set_cell(target, cell);
        board.check_promotion(target);
        board
    }

    fn check_promotion(&mut self, point: Point) {
        let cell = *self.get_cell(point);
        if !cell.is_queen() && (point.y == 0 || point.y == 7) {
            self.set_cell(point, cell.promote());
        }
    }

    fn get_captures(&self, start: Point) -> Vec<Route> {
        let mut board = self.clear_moves();
        let mut routes = vec![Route {
            points: vec![start],
        }];
        let mut captures = vec![start];
        let is_queen = board.get_cell(start).is_queen();
        while let Some(capture_point) = captures.shift() {
            let route = routes
                .iter()
                .find(|route| *route.last().unwrap() == capture_point)
                .unwrap()
                .clone();
            let enemy_neighbours = if is_queen {
                board
                    .route_capture(&route)
                    .get_enemy_neighbours_queen(capture_point)
            } else {
                board
                    .route_capture(&route)
                    .get_enemy_neighbours(capture_point)
            };
            let mut valid_captures: Vec<Point> = Vec::new();
            for neighbour_point in enemy_neighbours {
                let delta = neighbour_point.subtract(&capture_point).signum();
                if is_queen {
                    let mut start = neighbour_point.add(&delta);
                    while start.valid() && board.get_cell(start).is_empty() {
                        valid_captures.push(start);
                        start = start.add(&delta);
                    }
                } else {
                    let start = neighbour_point.add(&delta);
                    if start.valid() && board.get_cell(start).is_empty() {
                        valid_captures.push(start);
                    }
                }
            }

            valid_captures.into_iter().for_each(|point_behind_enemy| {
                board.set_cell(point_behind_enemy, Cell::Capture);
                captures.push(point_behind_enemy);
                routes.push(route.add_point(point_behind_enemy));
            });
        }
        routes
    }

    fn set_cell(&mut self, point: Point, cell: Cell) {
        self.cells[point.y as usize][point.x as usize] = cell;
    }

    fn get_cell(&self, point: Point) -> &Cell {
        &self.cells[point.y as usize][point.x as usize]
    }

    fn get_enemy_neighbours(&self, point: Point) -> Vec<Point> {
        let cell = self.get_cell(point);
        self.get_neighbours(point)
            .into_iter()
            .filter(|neighbour_point| self.get_cell(*neighbour_point).is_enemy(cell))
            .collect()
    }

    fn get_enemy_neighbours_queen(&self, point: Point) -> Vec<Point> {
        let cell = self.get_cell(point);
        self.get_neighbours_queen(point)
            .into_iter()
            .filter(|neighbour_point| self.get_cell(*neighbour_point).is_enemy(cell))
            .collect()
    }

    fn get_neighbours(&self, point: Point) -> Vec<Point> {
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

    fn get_neighbours_queen(&self, point: Point) -> Vec<Point> {
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

    fn clear_moves(&self) -> Board {
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
