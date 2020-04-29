use std::rc::Rc;
use std::sync::Mutex;
use std::time::Instant;

use crate::engine::keyboard_mover::KeyboardMover;
use crate::engine::{element::Element, elements_handler::ElementHandler};
use crate::engine::{sdl_handler, sdl_handler::SdlHandler};

pub struct GameLogic {}
pub trait Game {
    fn new() -> Self;
    fn run(&self) -> Result<(), String>;
}
impl Game for GameLogic {
    fn new() -> GameLogic {
        GameLogic {}
    }
    fn run(&self) -> Result<(), String> {
        let mut sdl_hnd: SdlHandler = SdlHandler::new("My own game", 800, 600, 60)?;
        let player = Rc::new(Mutex::new(Element::new(10, 10, 10, 10)));
        let player_mover = Rc::new(Mutex::new(KeyboardMover::new(player.clone())));
        let mut element_hnd = ElementHandler::new();
        element_hnd.elements.push(player.clone());

        //sdl_hnd.subcribe_movement()
        sdl_hnd.listeners.push(player_mover.clone());
        '_running: loop {
            sdl_hnd.clean_canvas();
            sdl_hnd.process_events();
            sdl_hnd.draw_elements(player.clone());
        }
    }
}
