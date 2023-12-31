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

    let board = Board::new();
    println!("{}", board);
}
const FOG_OF_WAR: char = '⬜';
/// The playing board that ships are placed on.
struct Board {
    cells: Vec<char>,
    width: usize,
    height: usize,
}

impl Board {
    fn new() -> Self {
        Board {
            cells: vec![FOG_OF_WAR; 100],
            width: 10,
            height: 10,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for i in 0..self.height {
            write!(f, "\t|")?;
            for j in 0..self.width {
                write!(f, "{}", &self.cells[i + j])?;
            }
            writeln!(f, "|")?;
        }
        writeln!(f)?;
        Ok(())
    }
}
