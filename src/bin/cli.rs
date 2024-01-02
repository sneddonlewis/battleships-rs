fn main() {
    if let Err(e) = battleships_rs::game::game_engine::Game::new().game_loop() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
