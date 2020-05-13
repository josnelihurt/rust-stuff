use core::cell::RefCell;
use std::rc::Rc;

use crate::engine::basic_types::*;
use crate::engine::element::Element;

use crate::engine::Mover;

pub struct KeyboardMover {
    parent: Rc<RefCell<Element>>,
    allowed_rect: Rect,
    speed: u32,
}
impl Mover for KeyboardMover {
    fn r#move(&mut self, m: Move) {
        let mut parent = self.parent.borrow_mut();
        trace!("x {} y {} ", parent.data.position.x, parent.data.position.y);
        match m {
            Move::Up => {
                if parent.data.position.y > self.allowed_rect.y {
                    parent.data.position.y = parent.data.position.y - self.speed as f32;
                }
            }
            Move::Down => {
                if parent.data.position.y < self.allowed_rect.y + self.allowed_rect.height as f32 {
                    parent.data.position.y = parent.data.position.y + self.speed as f32;
                }
            }
            Move::Right => {
                if parent.data.position.x < self.allowed_rect.x + self.allowed_rect.width as f32 {
                    parent.data.position.x = parent.data.position.x + self.speed as f32;
                }
            }
            Move::Left => {
                if parent.data.position.x > self.allowed_rect.x {
                    parent.data.position.x = parent.data.position.x - self.speed as f32;
                }
            }
        };
    }
}

impl KeyboardMover {
    pub fn new(
        parent: Rc<RefCell<Element>>,
        allowed_rect: Rect,
        speed: u32,
    ) -> Rc<RefCell<Box<dyn Mover>>> {
        Rc::new(RefCell::new(Box::new(KeyboardMover {
            parent: parent,
            allowed_rect: allowed_rect,
            speed: speed,
        })))
    }
}
