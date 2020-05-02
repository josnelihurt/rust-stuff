use core::cell::RefCell;
use std::rc::Rc;
use std::vec::Vec;

use crate::engine::element::Element;
use crate::engine::Renderer;

pub struct ElementHandler {
    elements: Vec<Rc<RefCell<Element>>>,
}

impl ElementHandler {
    pub fn new() -> ElementHandler {
        ElementHandler {
            elements: Vec::new(),
        }
    }
    pub fn add_element(&mut self, element: Rc<RefCell<Element>>) {
        self.elements.push(element);
    }
    pub fn draw(&self, renderer: &mut dyn Renderer) -> Result<(), String> {
        for element in self.elements.iter() {
            element.borrow().draw(renderer)?;
        }
        Ok(())
    }
    pub fn update_elements(&self) -> Result<(), String> {
        for element in self.elements.iter() {
            element.borrow_mut().update()?;
        }
        Ok(())
    }
}
