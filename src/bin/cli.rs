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
                print_winner();
            } else {
                print_gameover();
            }
        }
    }
}

fn cli_game_loop(mut game: Game) -> AppResult<bool> {
    while !game.is_gameover() {
        clear_screen();
        print_title();
        println!("{}", game);

        println!("Enter target coordinates:");
        let coords = read_coordinates();

        game.fire(coords)?;
    }

    clear_screen();
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

fn clear_screen() {
    std::process::Command::new("clear").status().unwrap();
}

fn print_title() {
    let title = r#"
         __       ___ ___       ___     __          __   __  
        |__)  /\   |   |  |    |__     /__` |__| | |__) /__` 
        |__) /~~\  |   |  |___ |___    .__/ |  | | |    .__/ 
"#;
    println!("{}", title);
}

fn print_gameover() {
    let title = r#"
         __              ___  __        ___  __  
        / _`  /\   |\/| |__  /  \ \  / |__  |__) 
        \__> /~~\  |  | |___ \__/  \/  |___ |  \
"#;
    println!("{}", title);
}

fn print_winner() {
    let title = r#"
                          ___  __  
        |  | | |\ | |\ | |__  |__) 
        |/\| | | \| | \| |___ |  \
"#;
    println!("{}", title);
}
