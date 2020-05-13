use core::cell::RefCell;
use std::rc::Rc;

use crate::config;
use crate::engine::{
    basic_types::*, element::Element, keyboard_mover::KeyboardMover,
    sprite_renderer::SpriteRenderer, DirectMedia,
};
pub mod texture {
    pub const PATH: &'static str = "res/sprites/player.png";
}

pub fn new(dm: &mut dyn DirectMedia) -> Rc<RefCell<Element>> {
    let size: u32 = 50;
    let initial_x: i32 = (config::screen::WIDTH / 2) - (size / 2) as i32;
    let initial_y: i32 = config::screen::HEIGHT - size as i32 - 50;
    let result = Element::new_shared(initial_x, initial_y, size, size, 180.0);
    let allowed_rect = Rect::new(
        (0 + size / 3) as i32,
        config::screen::HEIGHT / 2,
        config::screen::WIDTH - (7 / 3 * size) as i32,
        config::screen::HEIGHT / 2 - 5 / 3 * size as i32,
    );
    let mover = KeyboardMover::new(result.clone(), allowed_rect, 15);
    dm.subcribe_movement(mover.clone());
    let renderer = SpriteRenderer::new(String::from(texture::PATH));
    {
        let mut element = result.borrow_mut();
        element.add_component(renderer);
    }
    result
}
