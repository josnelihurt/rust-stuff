use core::cell::RefCell;
use std::rc::Rc;

use crate::config;
use crate::engine::{
    element::Element, sprite_renderer::SpriteRenderer, DirectMedia,
};

pub fn new_enemy(_dm: &mut dyn DirectMedia) -> Rc<RefCell<Element>> {
    let size: u32 = 30;
    let initial_x: i32 = (config::screen::WIDTH / 2) - (size / 2) as i32;
    let initial_y: i32 = 50;
    let enemy = Element::new_shared(initial_x, initial_y, size, size, 180.0);
    //let mover = KeyboardMover::new(enemy.clone());
    //dm.subcribe_movement(mover.clone());
    let renderer = SpriteRenderer::new(enemy.clone(), String::from("res/sprites/basic_enemy.png"));
    enemy.borrow_mut().add_component(renderer);
    enemy
}
