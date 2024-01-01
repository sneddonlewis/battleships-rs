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

    fn read_coordinates(&self) -> Coords {
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input).unwrap();
        let coords = input.trim().try_into();
        match coords {
            Ok(c) => c,
            Err(_) => self.read_coordinates(),
        }
    }

    pub fn game_loop(&mut self) -> AppResult<()> {
        while !self.is_won() {
            self.show_board();

            println!("Enter target coordinates in the format 'B3'");
            let coords = self.read_coordinates();

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
