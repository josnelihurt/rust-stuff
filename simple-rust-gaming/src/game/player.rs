use std::rc::Rc;
use std::sync::Mutex;

use crate::engine::keyboard_mover::KeyboardMover;
use crate::engine::sdl_handler::SdlHandler;
use crate::engine::{element::Element, elements_handler::ElementHandler, DirectMedia};

pub fn newPlayer(dm: &mut Box<dyn DirectMedia>) -> Rc<Mutex<Element>> {
    let player = Rc::new(Mutex::new(Element::new(10, 10, 10, 10)));
    let player_mover = Rc::new(Mutex::new(KeyboardMover::new(player.clone())));
    dm.subcribe_movement(player_mover.clone());
    player
}
