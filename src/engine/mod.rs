mod board;
mod cell;
mod checker;
mod route;
mod turn;

pub use board::Board;
pub use cell::Cell;
pub use checker::Checker;
pub use route::Route;
pub use turn::Turn;

use crate::utility::{Point, Shift};

pub struct Engine {}

impl Engine {
    pub fn make_move(board: Board, from: Point, to: Point) -> Board {
        let mut board = Engine::with_legal_moves(board, from);
        let to_cell = *board.get_cell(to);
        let from_cell = *board.get_cell(from);
        match to_cell {
            Cell::Move => {
                board.set_cell(from, Cell::Empty);
                board.set_cell(to, from_cell);
                board.check_promotion(to);
            }
            Cell::Capture => board = Engine::capture(&board, from, to),
            _ => {}
        }
        board.turn = board.turn.next();
        board.clear_moves()
    }

    pub fn with_legal_moves(board: Board, from: Point) -> Board {
        let mut board = board.clear_moves();
        let cell = *board.get_cell(from);
        if cell.is_queen() {
            let moves: Vec<Point> = board
                .get_neighbours_queen(from)
                .into_iter()
                .filter(|point| board.get_cell(*point).is_empty())
                .collect();
            moves
                .into_iter()
                .for_each(|point| board.set_cell(point, Cell::Move));
            Engine::get_captures(&board, from)
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
            Engine::get_captures(&board, from)
                .into_iter()
                .map(|route| route.get_after_last())
                .filter(|route| !route.is_empty())
                .for_each(|route| board.set_cell(*route.last().unwrap(), Cell::Capture));
        }
        board
    }

    pub fn capture(board: &Board, from: Point, target: Point) -> Board {
        let mut board = board.clone();
        Engine::get_captures(&board, from)
            .into_iter()
            .find(|route| *route.last().unwrap() == target)
            .unwrap_or_default()
            .iter()
            .reduce(|prev, curr| {
                board = Engine::simple_capture(&board, *prev, *curr);
                curr
            });
        board
    }

    fn route_capture(board: &Board, route: &Route) -> Board {
        let mut board = board.clone();
        route.iter().reduce(|prev, curr| {
            board = Engine::simple_capture(&board, *prev, *curr);
            curr
        });
        board
    }

    fn simple_capture(board: &Board, from: Point, target: Point) -> Board {
        let mut board = board.clone();
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
    fn get_captures(board: &Board, start: Point) -> Vec<Route> {
        let mut board = board.clear_moves();
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
                Engine::route_capture(&board, &route).get_enemy_neighbours_queen(capture_point)
            } else {
                Engine::route_capture(&board, &route).get_enemy_neighbours(capture_point)
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
}
