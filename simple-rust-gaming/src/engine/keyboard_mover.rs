use core::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

use crate::engine::basic_types::*;
use crate::engine::element::Element;
use crate::engine::Component;
use crate::engine::Mover;
use crate::engine::Renderer;

pub struct KeyboardMover {
    pub parent: Rc<RefCell<Element>>,
}
impl Mover for KeyboardMover {
    fn r#move(&mut self, m: Move) {
        match m {
            Move::Up => self.parent.borrow_mut().r#move(0, -10),
            Move::Down => self.parent.borrow_mut().r#move(0, 10),
            Move::Right => self.parent.borrow_mut().r#move(10, 0),
            Move::Left => self.parent.borrow_mut().r#move(-10, 0),
        };
    }
}

impl KeyboardMover {
    pub fn new(parent:  Rc<RefCell<Element>>) -> Rc<RefCell<Box<Mover>>> {
        Rc::new(RefCell::new(Box::new(KeyboardMover { parent: parent })))
    }
}
