use core::cell::RefCell;
use std::rc::Rc;

use crate::engine::basic_types::*;
use crate::engine::{element::Element, *};

pub struct SpriteRenderer {
    parent: Rc<RefCell<Element>>,
    path: String, // Idk how to store the sprite :/
}
impl SpriteRenderer {
    pub fn new(parent: Rc<RefCell<Element>>, path: String) -> RefCell<Box<dyn Component>> {
        RefCell::new(Box::new(SpriteRenderer {
            parent: parent,
            path: path,
        }))
    }
}
impl Component for SpriteRenderer {
    fn on_collision(&mut self) -> Result<(), String> {
        Ok(())
    }
}
impl Drawable for SpriteRenderer {
    fn draw(&self, renderer: &mut dyn Renderer) -> Result<(), String> {
        let position = self.parent.borrow().position.clone();
        let rotation = self.parent.borrow().rotation;
        renderer.copy(&self.path, &position, &Vec2D::new(50, 50), rotation)?;
        Ok(())
    }
}
impl Updatable for SpriteRenderer {
    fn update(&mut self) -> Result<(), String> {
        Ok(())
    }
}
