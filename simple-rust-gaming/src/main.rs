mod config;
mod engine;
mod game;

use engine::basic_types::Err;
use game::GameState;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting process");
    let mut game = GameState::new()?;
    match game.run() {
        Err(error) => {
            println!("{}", error);
            if error == Err::USER_EXIT.to_string() {
                println!("user request exit");
            }
        }
        _ => {}
    }
    println!("Exiting process");
    Ok(())
}
