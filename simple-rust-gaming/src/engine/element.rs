use crate::engine::basic_types::*;
use crate::engine::*;
use core::cell::RefCell;

use std::rc::Rc;

pub struct Element {
    pub active: bool,
    pub position: Vec2D,
    pub rotation: f64,
    pub components: Vec<RefCell<Box<dyn Component>>>,
}
impl Element {
    pub fn new(x: i32, y: i32, _size_x: u32, _size_y: u32, rotation: f64) -> Element {
        Element {
            active: false,
            position: Vec2D::new(x, y),
            rotation: rotation,
            components: Vec::new(),
        }
    }
    pub fn new_shared(
        x: i32,
        y: i32,
        _size_x: u32,
        _size_y: u32,
        rotation: f64,
    ) -> Rc<RefCell<Element>> {
        Rc::new(RefCell::new(Element::new(x, y, _size_x, _size_y, rotation)))
    }
    pub fn r#move<T: num::cast::AsPrimitive<f32>>(&mut self, dx: T, dy: T) {
        self.position.x += dx.as_();
        self.position.y += dy.as_();
    }
    pub fn add_component(&mut self, component: RefCell<Box<dyn Component>>) {
        self.components.push(component);
    }
}
impl Drawable for Element {
    fn draw(&self, renderer: &mut dyn Renderer) -> Result<(), String> {
        for item in self.components.iter() {
            item.borrow().on_draw(renderer)?;
        }
        Ok(())
    }
}
impl Updatable for Element {
    fn update(&mut self) -> Result<(), String> {
        self.rotation += 0.1;
        for item in self.components.iter() {
            item.borrow_mut().on_update()?;
        }
        Ok(())
    }
}
