use core::cell::RefCell;
use std::rc::Rc;

use crate::config;
use crate::engine::{
    element::Element, keyboard_mover::KeyboardMover, sprite_renderer::SpriteRenderer, DirectMedia,
};

pub fn new_player(dm: &mut Box<dyn DirectMedia>) -> Rc<RefCell<Element>> {
    let player_size: u32 = 30;
    let initial_x: i32 = (config::screen::WIDTH / 2) - (player_size / 2) as i32;
    let initial_y: i32 = config::screen::HEIGHT - player_size as i32 - 5;
    let player = Element::new_shared(initial_x, initial_y, player_size, player_size);
    let player_mover = KeyboardMover::new(player.clone());
    dm.subcribe_movement(player_mover.clone());
    let player_renderer = SpriteRenderer::new(player.clone());
    player.borrow_mut().add_component(player_renderer);
    player
}
