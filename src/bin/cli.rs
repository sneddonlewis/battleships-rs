use battleships_rs::{
    error::AppResult,
    game::{coordinates::Coords, game_engine::Game},
};
use std::io;

fn main() {
    match cli_game_loop(Game::new()) {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        Ok(won) => {
            if won {
                println!("winner")
            } else {
                println!("looser")
            }
        }
    }
}

fn cli_game_loop(mut game: Game) -> AppResult<bool> {
    while !game.is_gameover() {
        println!("{}", game);

        println!("Enter target coordinates:");
        let coords = read_coordinates();

        let fire_result = game.fire(coords)?;

        println!("fire result: {}", fire_result);
    }

    Ok(game.is_won())
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
