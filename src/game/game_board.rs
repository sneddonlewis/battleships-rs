use std::fmt::Display;

use crate::error::AppResult;
use crate::game::board_markers::{FOG_OF_WAR, MISS};
use crate::game::board_markers::{HIT, SHIP};
use crate::game::coordinates::Coords;

pub struct Board {
    cells: Vec<char>,
    pub fogged_cells: Vec<char>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Board {
            cells: vec![MISS; size],
            fogged_cells: vec![FOG_OF_WAR; size],
            width,
            height,
        }
    }
    pub fn default() -> Self {
        let mut board = Board::new(10, 10);
        board.cells[9] = SHIP;
        board
    }

    pub fn all_vessels_destroyed(&self) -> bool {
        !self.cells.contains(&SHIP)
    }

    pub fn try_fire(&mut self, coords: Coords) -> AppResult<bool> {
        let idx = self.width * coords.col_idx + coords.row_idx;
        if idx > self.cells.len() {
            return Err("coordinates fire off the board".into());
        }
        // unwrap as we've already checked index
        let target = self.cells.get(idx).unwrap();
        let fogged_target = self.fogged_cells.get(idx).unwrap();
        if *fogged_target != FOG_OF_WAR {
            return Err("coordinates aren't targetting fog of war".into());
        }
        if *target != SHIP {
            self.fogged_cells[idx] = MISS;
            return Ok(false);
        }
        if *target == SHIP {
            self.fogged_cells[idx] = HIT;
            self.cells[idx] = HIT;
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
