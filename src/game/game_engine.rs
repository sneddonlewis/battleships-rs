use std::fmt::Display;

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

    pub fn fire(&mut self, coords: Coords) -> AppResult<bool> {
        self.board.try_fire(coords)
    }

    pub fn is_gameover(&self) -> bool {
        self.is_won()
    }

    pub fn is_won(&self) -> bool {
        self.board.all_vessels_destroyed()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board)
    }
}
