mod enemy;
mod player;

use crate::config;
use crate::engine::element::Element;
use crate::engine::{elements_handler::ElementHandler, sdl_handler::SdlHandler, DirectMedia};

pub struct GameState {
    element_hnd: ElementHandler<Element>,
    direct_media: Box<dyn DirectMedia>,
}
impl GameState {
    pub fn new() -> Result<GameState, String> {
        Ok(GameState {
            element_hnd: ElementHandler::new(),
            direct_media: Box::new(SdlHandler::new(
                config::TITLE,
                config::screen::WIDTH as u32,
                config::screen::HEIGHT as u32,
                60,
            )?),
        })
    }
    pub fn run(&mut self) -> Result<(), String> {
        self.initializa_elements();
        '_running: loop {
            self.direct_media.clean_canvas();
            self.direct_media.process_events()?;
            self.element_hnd.updgitate()?;
            self.direct_media.draw_elements(&self.element_hnd)?;
            self.direct_media.present();
        }
    }
}
impl GameState {
    fn initializa_elements(&mut self) {
        self.element_hnd
            .add_element(player::new_player(&mut self.direct_media));
        self.element_hnd
            .add_element(enemy::new_enemy(&mut self.direct_media));
    }
}
