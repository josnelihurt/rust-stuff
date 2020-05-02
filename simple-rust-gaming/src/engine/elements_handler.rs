use core::cell::RefCell;
use std::vec::Vec;

use crate::engine::element::Element;

use std::rc::Rc;
use std::sync::Mutex;

pub struct ElementHandler {
    pub elements: Vec<Rc<RefCell<Element>>>,
}

impl ElementHandler {
    pub fn new() -> ElementHandler {
        ElementHandler {
            elements: Vec::new(),
        }
    }
}
