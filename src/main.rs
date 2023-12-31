#![allow(dead_code)]
use std::fmt::Display;

fn main() {
    println!("Battleships");
    // AircraftCarrier 5
    // BattleShip 4
    // Submarine 3
    // Cruiser 3
    // Destroyer 2

    // properties of a ship
    // cells;
    // name;
    // isSunk;
    // isHorizontal;
    // cellsHit;
    // Coord start;
    // Coord end;
    // sunkCount = 0;

    Game::new().show_board();
}
const FOG_OF_WAR: char = '~';
const MISS: char = 'M';
const HIT: char = 'X';
const SHIP: char = 'O';

struct Game {
    board: Board,
}

impl Game {
    fn new() -> Self {
        Game {
            board: Board::default(),
        }
    }

    fn show_board(self) {
        println!("{}", self.board);
    }
}

struct Board {
    cells: Vec<char>,
    fogged_cells: Vec<char>,
    width: u32,
    height: u32,
}

impl Board {
    fn new(width: u32, height: u32) -> Self {
        let size = width * height;
        Board {
            cells: vec![MISS; size as usize],
            fogged_cells: vec![FOG_OF_WAR; size as usize],
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

        writeln!(f)?;

        for i in 0..self.height as usize {
            write!(f, "\t")?;
            for j in 0..self.width as usize {
                write!(f, "{} ", &self.fogged_cells[(i * 10) + j])?;
            }

            write!(f, "\t")?;
            for j in 0..self.width as usize {
                write!(f, "{} ", &self.cells[(i * 10) + j])?;
            }
            writeln!(f)?;
        }

        write!(f, "\t")?;
        for _ in 0..self.width {
            write!(f, "- ")?;
        }
        write!(f, "\t")?;
        for _ in 0..self.width {
            write!(f, "- ")?;
        }
        writeln!(f)?;

        write!(f, "\t")?;
        for i in 0..self.width {
            let c = char::from_u32(i + 65).unwrap();
            write!(f, "{} ", c)?;
        }
        write!(f, "\t")?;
        for i in 0..self.width {
            let c = char::from_u32(i + 65).unwrap();
            write!(f, "{} ", c)?;
        }

        writeln!(f)?;
        Ok(())
    }
}
