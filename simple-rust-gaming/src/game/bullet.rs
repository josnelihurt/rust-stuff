use core::cell::RefCell;
use std::rc::Rc;

use crate::config;
use crate::engine::{element::Element, sprite_renderer::SpriteRenderer, DirectMedia};

pub mod texture {
    pub const PATH: &'static str = "res/sprites/bullet.png";
}
pub fn new(_dm: &mut dyn DirectMedia) -> Rc<RefCell<Element>> {
    let size: u32 = 30;
    let initial_x: i32 = (config::screen::WIDTH / 2) - (size / 2) as i32;
    let initial_y: i32 = config::screen::HEIGHT / 2;
    let result = Element::new_shared(initial_x, initial_y, size, size, 180.0);
    let renderer = SpriteRenderer::new(result.clone(), String::from(texture::PATH));
    result.borrow_mut().add_component(renderer);
    result
}
