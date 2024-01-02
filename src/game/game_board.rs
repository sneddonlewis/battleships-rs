use std::fmt::Display;

use crate::error::AppResult;
use crate::game::coordinates::Coords;

use crate::game::board_markers::BoardMarker;
use crate::vessels::Vessel;

pub struct Board {
    cells: Vec<BoardMarker>,
    pub fogged_cells: Vec<BoardMarker>,
    vessels: Vec<Vessel>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Board {
            cells: vec![BoardMarker::Miss; size],
            fogged_cells: vec![BoardMarker::FogOfWar; size],
            vessels: Vec::new(),
            width,
            height,
        }
    }
    pub fn default() -> Self {
        let mut board = Board::new(10, 10);
        board.place_ships();
        board
    }

    fn place_ships(&mut self) {
        self.place_vessel(vec![5]);
        self.place_vessel(vec![83, 84]);
        self.place_vessel(vec![77, 78, 79]);
        self.place_vessel(vec![41, 42, 43, 44]);
        self.place_vessel(vec![11, 12, 13, 14]);
    }

    fn place_vessel(&mut self, position: Vec<usize>) -> bool {
        if !self.is_space_for_vessel(&position) {
            return false;
        }
        let vessel = Vessel::try_place(position.clone()).unwrap();
        for p in position.iter() {
            self.cells[*p] = BoardMarker::Ship;
        }
        self.vessels.push(vessel);
        true
    }

    fn is_space_for_vessel(&self, position: &Vec<usize>) -> bool {
        for pos in position {
            if self.cells[*pos] == BoardMarker::Ship {
                return false;
            }
        }
        true
    }

    pub fn all_vessels_destroyed(&self) -> bool {
        !self.cells.contains(&BoardMarker::Ship)
    }

    pub fn try_fire(&mut self, coords: Coords) -> AppResult<bool> {
        let idx: usize = coords.into();
        if idx > self.cells.len() {
            return Err("coordinates fire off the board".into());
        }
        for vessel in self.vessels.iter_mut() {
            let hit = vessel.fire(idx);
            if hit {
                self.fogged_cells[idx] = BoardMarker::Hit;
                self.cells[idx] = BoardMarker::Hit;
                return Ok(true);
            }
        }
        self.fogged_cells[idx] = BoardMarker::Miss;
        self.cells[idx] = BoardMarker::Miss;
        Ok(false)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;

        for v in self.vessels.iter() {
            writeln!(f, "{}", v)?;
        }

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
