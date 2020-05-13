use crate::engine::basic_types::*;
use crate::engine::*;
use core::cell::RefCell;

use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct ElementData {
    pub active: bool,
    pub position: Rect,
    pub rotation: f64,
}
pub struct Element {
    pub data: ElementData,
    pub components: Vec<RefCell<Box<dyn Component>>>,
}
impl Element {
    pub fn new(x: i32, y: i32, size_x: u32, size_y: u32, rotation: f64) -> Element {
        Element {
            data: ElementData {
                active: false,
                position: Rect::new(x, y, size_x, size_y),
                rotation: rotation,
            },
            components: Vec::new(),
        }
    }
    pub fn new_shared(
        x: i32,
        y: i32,
        size_x: u32,
        size_y: u32,
        rotation: f64,
    ) -> Rc<RefCell<Element>> {
        Rc::new(RefCell::new(Element::new(x, y, size_x, size_y, rotation)))
    }
    pub fn r#move<T: num::cast::AsPrimitive<f32>>(&mut self, dx: T, dy: T) {
        self.data.position.x += dx.as_();
        self.data.position.y += dy.as_();
    }
    pub fn add_component(&mut self, component: RefCell<Box<dyn Component>>) {
        self.components.push(component);
    }
}
impl Drawable for Element {
    fn draw(&self, renderer: &mut dyn Renderer) -> Result<(), String> {
        for component in self.components.iter() {
            component.borrow().on_draw(self, renderer)?;
        }
        Ok(())
    }
}
impl Updatable for Element {
    fn update(&mut self) -> Result<(), String> {
        for component in self.components.iter() {
            match component.borrow().on_update(self)? {
                Some(data) => self.data = data,
                _ => {}
            }
        }
        Ok(())
    }
}
