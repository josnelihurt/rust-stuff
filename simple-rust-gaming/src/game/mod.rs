mod bullet;
pub mod bullet_mover;
pub mod player_shooter;
mod enemy;
mod player;

use crate::config;
use crate::engine::element::Element;
use crate::engine::Updatable;
use crate::engine::{
    elements_handler::ElementHandler,
    sdl_handler::{SdlContext, SdlHandler, TexturesCache},
    DirectMedia,
};
use log::{trace};
use std::path::PathBuf;
use std::{fs, io};

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
        //Fill up textures
        let creator = ctx.canvas.texture_creator();
        let mut texture_cache = TexturesCache::new();
        let list_sprites = || -> io::Result<Vec<PathBuf>> {
            let entries = fs::read_dir("res/sprites/")?
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()?;
            Ok(entries)
        };
        let sprites = list_sprites();
        match sprites {
            Ok(list) => {
                for img in list {
                    let value: String = img.into_os_string().into_string().unwrap();
                    trace!("loading ... {}", value);
                    texture_cache.load_texture(value, &creator)?;
                }
            }
            Err(error) => return Err(format!("invalid resources {}", error)),
        }

        //Create SDL HND
        let mut direct_media = SdlHandler::new(&mut ctx, &mut texture_cache, 30);
        self.initializa_elements(&mut direct_media);

        '_running: loop {
            direct_media.clean_canvas();
            let events = direct_media.process_events()?;
            self.element_hnd.update(&events)?;
            direct_media.draw_elements(&self.element_hnd)?;
            direct_media.present();
        }
    }
}
impl GameState {
    fn initializa_elements(&mut self, dm: &mut dyn DirectMedia) {
        self.element_hnd.add_element(player::new(dm));
        self.element_hnd.add_element(enemy::new(dm));
        self.element_hnd.add_element(bullet::new(dm));
    }
}
