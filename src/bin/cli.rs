use battleships_rs::{error::AppResult, game::game_engine::Game};

fn main() {
    if let Err(e) = cli_game_loop(Game::new()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn cli_game_loop(mut game: Game) -> AppResult<()> {
    game.game_loop()
}
