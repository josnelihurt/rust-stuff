use std::rc::Rc;
use std::sync::Mutex;

use crate::config;
use crate::engine::keyboard_mover::KeyboardMover;
use crate::engine::{element::Element, DirectMedia};

pub fn new_player(dm: &mut Box<dyn DirectMedia>) -> Rc<Mutex<Element>> {
    let player_size: u32 = 30;
    let initial_x: i32 = (config::screen::WIDTH / 2) - (player_size / 2) as i32;
    let initial_y: i32 = config::screen::HEIGHT - player_size as i32 - 5;
    let player = Rc::new(Mutex::new(Element::new(
        initial_x,
        initial_y,
        player_size,
        player_size,
    )));
    let player_mover = Rc::new(Mutex::new(KeyboardMover::new(player.clone())));
    dm.subcribe_movement(player_mover.clone());
    player.lock().unwrap().components.push(player_mover.clone());
    player
}
