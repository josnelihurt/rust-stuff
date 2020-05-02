use crate::engine::basic_types::Vec2D;
use crate::engine::Component;
use crate::engine::Renderer;
use core::cell::RefCell;

use std::rc::Rc;


pub struct Element {
    pub active: bool,
    pub position: Vec2D,
    pub rotation: f32,
    pub components: Vec<RefCell<Box<dyn Component>>>,
}
impl Element {
    pub fn new(x: i32, y: i32, _size_x: u32, _size_y: u32) -> Element {
        Element {
            active: false,
            position: Vec2D::new(x, y),
            rotation: 0.0,
            components: Vec::new(),
        }
    }
    pub fn new_shared(x: i32, y: i32, _size_x: u32, _size_y: u32) -> Rc<RefCell<Element>> {
        Rc::new(RefCell::new(Element::new(x, y, _size_x, _size_y)))
    }
    pub fn r#move<T: num::cast::AsPrimitive<f32>>(&mut self, dx: T, dy: T) {
        self.position.x += dx.as_();
        self.position.y += dy.as_();
    }
    pub fn draw(&self, renderer: &mut dyn Renderer) -> Result<bool, String> {
        for item in self.components.iter() {
            item.borrow().on_draw(renderer)?;
        }
        Ok(true)
    }
    pub fn update(&mut self) -> Result<bool, String> {
        for item in self.components.iter() {
            item.borrow_mut().on_update()?;
        }
        Ok(true)
    }
    pub fn add_component(&mut self,component: RefCell<Box<dyn Component>>){
        self.components.push(component);
    }
}
