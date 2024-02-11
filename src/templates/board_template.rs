use askama::Template;
use serde::{Deserialize, Serialize};

use crate::{engine::Board, utility::Point};

use super::CellTemplate;

#[derive(Deserialize, Template, Serialize, Default)]
#[template(path = "components/board.html")]
pub struct BoardTemplate {
    pub id: String,
    pub cells: Vec<Vec<CellTemplate>>,
    pub selected_point: Option<Point>,
}

impl BoardTemplate {
    pub fn is_selected(&self, x: &usize, y: &usize) -> bool {
        self.selected_point
            .map_or(false, |p| p.x == *x as i8 && p.y == *y as i8)
    }

    pub fn new(board: &Board, id: String, selected_point: Option<Point>) -> Self {
        

        let mut cells: Vec<Vec<CellTemplate>> =
            vec![vec![CellTemplate::default(); board.size]; board.size];
        for y in 0..board.size {
            for x in 0..board.size {
                cells[y][x] = CellTemplate {
                    cell: *board.get_cell(Point {
                        x: x as i8,
                        y: y as i8,
                    }),
                    x,
                    y,
                    id: id.clone(),
                    turn: board.turn,
                    is_selected: false,
                    selected_point,
                };
            }
        }
        Self {
            id,
            selected_point,
            cells,
        }
    }
}
