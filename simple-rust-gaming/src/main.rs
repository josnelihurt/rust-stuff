#[allow(dead_code)]
#[allow(unused_imports)]

mod game;
mod engine;

use std::error::Error;
use game::Game;


fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting process");
    let game: game::GameLogic = Game::new();
    game.run();
    println!("Exiting process");
    Ok(())
}
