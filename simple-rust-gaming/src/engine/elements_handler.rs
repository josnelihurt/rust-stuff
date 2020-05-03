use core::cell::RefCell;
use std::rc::Rc;
use std::vec::Vec;

use crate::engine::*;

pub struct ElementHandler<T>
where
    T: Drawable,
    T: Updatable,
{
    elements: Vec<Rc<RefCell<T>>>,
}

impl<T: Drawable + Updatable> ElementHandler<T> {
    pub fn new() -> ElementHandler<T> {
        ElementHandler {
            elements: Vec::new(),
        }
    }
    pub fn add_element(&mut self, element: Rc<RefCell<T>>) {
        self.elements.push(element);
    }
}
impl<T: Drawable + Updatable> Drawable for ElementHandler<T> {
    fn draw(&self, renderer: &mut dyn Renderer) -> Result<(), String> {
        for element in self.elements.iter() {
            element.borrow().draw(renderer)?;
        }
        Ok(())
    }
}
impl<T: Drawable + Updatable> Updatable for ElementHandler<T> {
    fn update(&mut self) -> Result<(), String> {
        for element in self.elements.iter() {
            element.borrow_mut().update()?;
        }
        Ok(())
    }
}
