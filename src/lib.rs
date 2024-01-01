#![allow(dead_code)]

mod error;
mod game;

use std::io;

use crate::error::AppResult;
use crate::game::{coordinates::Coords, game_engine::Game};

pub enum VesselType {
    Helicopter,
}

impl VesselType {
    fn size(&self) -> usize {
        match self {
            VesselType::Helicopter => 1,
        }
    }
}

pub fn run() -> AppResult<()> {
    println!("Battleships");
    // AircraftCarrier 5
    // BattleShip 4
    // Submarine 3
    // Cruiser 3
    // Destroyer 2
    // Helicopter 1

    // properties of a ship
    // cells;
    // name;
    // isSunk;
    // isHorizontal;
    // cellsHit;
    // Coord start;
    // Coord end;
    // sunkCount = 0;

    // TODOs
    // parse fire command
    // limited shots
    // add vessels to board

    let mut game = Game::new();

    let is_won = game.is_won();
    println!("Is won: {}", is_won);

    game.show_board();
    let mut input = String::new();
    println!("input fire coords");
    let _ = io::stdin().read_line(&mut input)?;

    let coords: Coords = input.trim().try_into()?;

    println!("{:?}", coords);

    println!("firing");
    let fire_result = game.fire(coords)?;

    println!("fire result: {}", fire_result);

    let is_won = game.is_won();
    println!("Is won: {}", is_won);

    game.show_board();

    Ok(())
}
