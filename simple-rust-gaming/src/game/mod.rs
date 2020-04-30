use std::rc::Rc;
use std::sync::Mutex;

mod player;

use crate::config;
use crate::engine::sdl_handler::SdlHandler;
use crate::engine::{element::Element, elements_handler::ElementHandler, DirectMedia};

pub struct GameLogic {
    element_hnd: ElementHandler,
    direct_media: Box<dyn DirectMedia>,
}
pub trait Game {
    fn new() -> Self;
    fn run(&mut self) -> Result<(), String>;
}
impl Game for GameLogic {
    fn new() -> GameLogic {
        GameLogic {
            element_hnd: ElementHandler::new(),
            direct_media: Box::new(SdlHandler::new(
                "My own game",
                config::screen::WIDTH as u32,
                config::screen::HEIGHT as u32,
                60,
            )),
        }
    }
    fn run(&mut self) -> Result<(), String> {
        self.initializa_elements();
        '_running: loop {
            self.direct_media.clean_canvas();
            self.direct_media.process_events()?;
            self.direct_media
                .draw_elements(self.element_hnd.elements[0].clone());
        }
    }
}
impl GameLogic {
    fn initializa_elements(&mut self) {
        let player: Rc<Mutex<Element>> = player::new_player(&mut self.direct_media);
        self.element_hnd.elements.push(player.clone());
    }
}
