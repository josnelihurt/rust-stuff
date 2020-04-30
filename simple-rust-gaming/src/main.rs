mod config;
mod engine;
mod game;

use game::Game;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting process");
    let mut game: game::GameLogic = Game::new();
    match game.run() {
        Err(error) => {
            if error == "Exit from user".to_string() {
                println!("user request exit");
            }
        }
        _ => {}
    }
    println!("Exiting process");
    Ok(())
}
