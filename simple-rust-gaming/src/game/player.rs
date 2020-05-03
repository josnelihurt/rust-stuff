use core::cell::RefCell;
use std::rc::Rc;

use crate::config;
use crate::engine::{
    element::Element, keyboard_mover::KeyboardMover, sprite_renderer::SpriteRenderer, DirectMedia,
};
pub mod texture {
    pub const PATH: &'static str = "res/sprites/player.png";
}

pub fn new_player(dm: &mut dyn DirectMedia) -> Rc<RefCell<Element>> {
    let size: u32 = 30;
    let initial_x: i32 = (config::screen::WIDTH / 2) - (size / 2) as i32;
    let initial_y: i32 = config::screen::HEIGHT - size as i32 - 50;
    let player = Element::new_shared(initial_x, initial_y, size, size, 180.0);
    let mover = KeyboardMover::new(player.clone());
    dm.subcribe_movement(mover.clone());
    let renderer = SpriteRenderer::new(player.clone(), String::from(texture::PATH));
    player.borrow_mut().add_component(renderer);
    player
}
