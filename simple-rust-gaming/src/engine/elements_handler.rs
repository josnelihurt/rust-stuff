use std::vec::Vec;

use crate::engine::*;

pub struct ElementHandler<T>
where
    T: Drawable,
    T: Updatable,
{
    elements: Vec<T>,
}

impl<T: Drawable + Updatable> ElementHandler<T> {
    pub fn new() -> ElementHandler<T> {
        ElementHandler {
            elements: Vec::new(),
        }
    }
    pub fn add_element(&mut self, element: T) {
        self.elements.push(element);
    }
    pub fn add_elements(&mut self, elements: &mut Vec<T>) {
        self.elements.append(elements);
    }
}
impl<T: Drawable + Updatable> Drawable for ElementHandler<T> {
    fn draw(&self, renderer: &mut dyn Renderer) -> Result<(), String> {
        for element in self.elements.iter() {
            element.draw(renderer)?;
        }
        Ok(())
    }
}
impl<T: Drawable + Updatable> Updatable for ElementHandler<T> {
    fn update(&mut self, events: &Vec<Event>) -> Result<(), String> {
        for element in self.elements.iter_mut() {
            element.update(events)?;
        }
        Ok(())
    }
}
