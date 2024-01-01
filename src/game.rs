use crate::error::{AppError, AppResult};

use std::fmt::Display;

pub const FOG_OF_WAR: char = '~';
pub const MISS: char = 'M';
pub const HIT: char = 'X';
pub const SHIP: char = 'O';

#[derive(Debug)]
pub struct Coords {
    row_idx: usize,
    col_idx: usize,
}

impl TryFrom<&str> for Coords {
    type Error = AppError;
    fn try_from(_value: &str) -> Result<Self, Self::Error> {
        Ok(Coords {
            row_idx: 9,
            col_idx: 0,
        })
    }
}

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        let mut board = Board::default();
        board.cells[9] = SHIP;
        Game { board }
    }

    pub fn show_board(&self) {
        println!("{}", self.board);
    }

    pub fn fire(&mut self, coords: Coords) -> AppResult<bool> {
        let idx = self.board.width * coords.col_idx + coords.row_idx;
        if idx > self.board.cells.len() {
            return Err("coordinates fire off the board".into());
        }
        // unwrap as we've already checked index
        let target = self.board.cells.get(idx).unwrap();
        let fogged_target = self.board.fogged_cells.get(idx).unwrap();
        if *fogged_target != '~' {
            return Err("coordinates aren't targetting fog of war".into());
        }
        if *target == 'M' {
            self.board.fogged_cells[idx] = 'M';
            return Ok(false);
        }
        if *target == 'O' {
            self.board.fogged_cells[idx] = 'X';
            return Ok(true);
        }
        panic!("expected unreachable when firing. Coordinate/target combination not covered");
    }

    pub fn is_won(&self) -> bool {
        self.board.cells.contains(&MISS)
    }
}

struct Board {
    cells: Vec<char>,
    fogged_cells: Vec<char>,
    width: usize,
    height: usize,
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Board {
            cells: vec![MISS; size],
            fogged_cells: vec![FOG_OF_WAR; size],
            width,
            height,
        }
    }
    fn default() -> Self {
        Board::new(10, 10)
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
