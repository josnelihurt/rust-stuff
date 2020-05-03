mod config;
mod engine;
mod game;

use core::cell::RefCell;
use engine::basic_types::Err;
use game::GameState;
use sdl2::image::LoadTexture;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::EventPump;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting process");
    let mut game = GameState::new()?;
    match game.run() {
        Err(error) => {
            if error == Err::USER_EXIT.to_string() {
                println!("user request exit");
            }
        }
        _ => {}
    }
    println!("Exiting process");
    Ok(())
}
