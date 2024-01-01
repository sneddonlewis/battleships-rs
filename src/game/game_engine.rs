use std::io;

use crate::error::AppResult;
use crate::game::coordinates::Coords;
use crate::game::game_board::Board;

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::default(),
        }
    }

    pub fn game_loop(&mut self) -> AppResult<()> {
        while !self.is_won() {
            self.show_board();
            let mut input = String::new();
            println!("input fire coords");
            let _ = io::stdin().read_line(&mut input)?;

            let coords: Coords = input.trim().try_into()?;

            println!("{:?}", coords);

            println!("firing");
            let fire_result = self.fire(coords)?;

            println!("fire result: {}", fire_result);

            let is_won = self.is_won();
            println!("Is won: {}", is_won);

            self.show_board();
        }

        Ok(())
    }

    pub fn show_board(&self) {
        println!("{}", self.board);
    }

    pub fn fire(&mut self, coords: Coords) -> AppResult<bool> {
        self.board.try_fire(coords)
    }

    pub fn is_won(&self) -> bool {
        self.board.all_vessels_destroyed()
    }
}
