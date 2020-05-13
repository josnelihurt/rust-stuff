mod config;
mod engine;
mod game;

#[macro_use]
extern crate deferred;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use engine::basic_types::err;
use std::env;
use game::GameState;
use log::{info, warn};

fn main() -> Result<(), String> {
    let key = "RUST_LOG";
    env::set_var(key, "trace");
    pretty_env_logger::init();
    info!("Starting process");
    let mut game = GameState::new()?;
    match game.run() {
        Err(error) => {
            if error == err::USER_EXIT.to_string() {
                warn!("user request exit");
            } else {
                error!("{}", error);
                return Err(error);
            }
        }
        _ => {}
    }
    info!("Exiting process");
    Ok(())
}
