mod enemy;
mod player;

use crate::config;
use crate::engine::element::Element;
use crate::engine::Updatable;
use crate::engine::{elements_handler::ElementHandler, sdl_handler::*, DirectMedia};

pub struct GameState {
    element_hnd: ElementHandler<Element>,
}
impl GameState {
    pub fn new() -> Result<GameState, String> {
        Ok(GameState {
            element_hnd: ElementHandler::new(),
        })
    }
    pub fn run(&mut self) -> Result<(), String> {
        let mut ctx = SdlContext::new(
            config::TITLE,
            config::screen::WIDTH as u32,
            config::screen::HEIGHT as u32,
        )?;
        let mut texture_cache = TexturesCache::new();
        let mut direct_media = SdlHandler::new(&mut ctx, &mut texture_cache, 30);
        self.initializa_elements(&mut direct_media);

        '_running: loop {
            direct_media.clean_canvas();
            direct_media.process_events()?;
            self.element_hnd.update()?;
            direct_media.draw_elements(&self.element_hnd)?;
            direct_media.present();
        }
    }
}
impl GameState {
    fn initializa_elements(&mut self, direct_media: &mut dyn DirectMedia) {
        self.element_hnd
            .add_element(player::new_player(direct_media));
        self.element_hnd.add_element(enemy::new_enemy(direct_media));
    }
}
