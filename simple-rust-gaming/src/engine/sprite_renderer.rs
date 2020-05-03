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
        let pos = self.parent.borrow().position.clone();
        let rot = self.parent.borrow().rotation;
        let rect = Rect::new(pos.x, pos.y, 50, 50);
        renderer.copy(&self.path, rect, rot)?;
        Ok(())
    }
}
impl Updatable for SpriteRenderer {
    fn update(&mut self) -> Result<(), String> {
        Ok(())
    }
}
