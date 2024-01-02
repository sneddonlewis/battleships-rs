use battleships_rs::{
    error::AppResult,
    game::{coordinates::Coords, game_engine::Game},
};
use std::io;

fn main() {
    if let Err(e) = cli_game_loop(Game::new()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn cli_game_loop(mut game: Game) -> AppResult<()> {
    while !game.is_won() {
        game.show_board();

        println!("Enter target coordinates:");
        let coords = read_coordinates();

        let fire_result = game.fire(coords)?;

        println!("fire result: {}", fire_result);
    }

    Ok(())
}

fn read_coordinates() -> Coords {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input).unwrap();
    let coords = input.trim().try_into();
    match coords {
        Ok(c) => c,
        Err(_) => read_coordinates(),
    }
}
