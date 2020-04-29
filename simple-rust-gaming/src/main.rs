mod engine;
#[allow(dead_code)]
#[allow(unused_imports)]
mod game;

use game::Game;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting process");
    let game: game::GameLogic = Game::new();
    game.run()?;
    println!("Exiting process");
    Ok(())
}
