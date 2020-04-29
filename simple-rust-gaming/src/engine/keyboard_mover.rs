use std::rc::Rc;
use std::sync::Mutex;

use crate::engine::basic_types::Move;
use crate::engine::element::Element;
use crate::engine::Component;
use crate::engine::Mover;

pub struct KeyboardMover {
    pub parent: Rc<Mutex<Element>>,
}

impl Component for KeyboardMover {
    fn on_update(&mut self) -> Result<bool, String> {
        Ok(true)
    }
    fn on_draw(&mut self) -> Result<bool, String> {
        Ok(true)
    }
    fn on_collision(&mut self) -> Result<bool, String> {
        Ok(true)
    }
}
impl Mover for KeyboardMover {
    fn r#move(&mut self, m: Move) {
        match m {
            Move::Up => self.parent.lock().unwrap().r#move(0, -10),
            Move::Down => self.parent.lock().unwrap().r#move(0, 10),
            Move::Right => self.parent.lock().unwrap().r#move(10, 0),
            Move::Left => self.parent.lock().unwrap().r#move(-10, 0),
        };
    }
}

impl KeyboardMover {
    pub fn new(parent: Rc<Mutex<Element>>) -> KeyboardMover {
        KeyboardMover { parent: parent }
    }
}
