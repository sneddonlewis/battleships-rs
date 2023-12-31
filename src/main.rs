use std::process::exit;

fn main() {
    if let Err(e) = battleships_rs::run() {
        eprintln!("{}", e);
        exit(1);
    }
}
