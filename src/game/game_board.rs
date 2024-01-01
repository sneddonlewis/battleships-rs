use std::fmt::Display;

use crate::error::AppResult;
use crate::game::coordinates::Coords;

use crate::game::board_markers::BoardMarker;

pub struct Board {
    cells: Vec<BoardMarker>,
    pub fogged_cells: Vec<BoardMarker>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Board {
            cells: vec![BoardMarker::Miss; size],
            fogged_cells: vec![BoardMarker::FogOfWar; size],
            width,
            height,
        }
    }
    pub fn default() -> Self {
        let mut board = Board::new(10, 10);
        board.cells[9] = BoardMarker::Ship;
        board
    }

    pub fn all_vessels_destroyed(&self) -> bool {
        !self.cells.contains(&BoardMarker::Ship)
    }

    pub fn try_fire(&mut self, coords: Coords) -> AppResult<bool> {
        let idx = self.height * coords.row_idx + coords.col_idx;
        if idx > self.cells.len() {
            return Err("coordinates fire off the board".into());
        }
        // unwrap as we've already checked index
        let target = self.cells.get(idx).unwrap();
        let fogged_target = self.fogged_cells.get(idx).unwrap();
        if fogged_target != &BoardMarker::FogOfWar {
            return Err("coordinates aren't targetting fog of war".into());
        }
        if target != &BoardMarker::Ship {
            self.fogged_cells[idx] = BoardMarker::Miss;
            return Ok(false);
        }
        if target == &BoardMarker::Ship {
            self.fogged_cells[idx] = BoardMarker::Hit;
            self.cells[idx] = BoardMarker::Hit;
            return Ok(true);
        }
        panic!("expected unreachable when firing. Coordinate/target combination not covered");
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;

        for i in 0..self.height as usize {
            let y_axis_coord = self.height as usize - i;
            write!(f, "\t{:3} |", y_axis_coord)?;
            for j in 0..self.width as usize {
                write!(f, "{} ", &self.fogged_cells[(i * 10) + j])?;
            }

            let y_axis_coord = self.height as usize - i;
            write!(f, "\t{:3} |", y_axis_coord)?;
            for j in 0..self.width as usize {
                write!(f, "{} ", &self.cells[(i * 10) + j])?;
            }
            writeln!(f)?;
        }

        // underline fogged cells
        write!(f, "\t     ")?;
        for _ in 0..self.width {
            write!(f, "- ")?;
        }
        // underline hidden cells
        write!(f, "\t     ")?;
        for _ in 0..self.width {
            write!(f, "- ")?;
        }
        writeln!(f)?;

        // coordinate letters on fogged cells
        write!(f, "\t     ")?;
        for i in 0..self.width {
            let c = char::from_u32((i + 65) as u32).unwrap();
            write!(f, "{} ", c)?;
        }
        // coordinate letters on hidden cells
        write!(f, "\t     ")?;
        for i in 0..self.width {
            let c = char::from_u32((i + 65) as u32).unwrap();
            write!(f, "{} ", c)?;
        }

        writeln!(f)?;
        Ok(())
    }
}
