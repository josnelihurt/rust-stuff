use crate::engine::element::Element;
use crate::engine::element::ElementData;
use crate::engine::Component;
use crate::engine::Drawable;
use crate::engine::Renderer;
use crate::engine::Updatable;
use core::cell::RefCell;
use std::rc::Rc;

pub struct BulletMover {
    parent: Rc<RefCell<Element>>,
    speed: f32,
}

impl Component for BulletMover {
    fn on_collision(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn on_update(&self, parent: &Element) -> Result<Option<ElementData>, String> {
        let mut element = parent.data.clone();
        element.position.y = element.position.y + self.speed;
        Ok(Some(element))
    }
    fn on_draw(&self, _: &Element, _: &mut dyn Renderer) -> Result<(), String> {
        Ok(())
    }
}
impl BulletMover {
    pub fn new(parent: Rc<RefCell<Element>>, speed: f32) -> RefCell<Box<dyn Component>> {
        RefCell::new(Box::new(BulletMover {
            parent: parent,
            speed: speed,
        }))
    }
}
