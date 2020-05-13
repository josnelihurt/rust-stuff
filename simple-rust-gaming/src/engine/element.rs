use crate::engine::basic_types::*;
use crate::engine::*;


#[derive(Clone, Debug)]
pub struct ElementData {
    pub active: bool,
    pub position: Rect,
    pub rotation: f64,
}
pub struct Element {
    pub data: ElementData,
    pub components: Vec<Box<dyn Component>>,
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
    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }
}
impl Drawable for Element {
    fn draw(&self, renderer: &mut dyn Renderer) -> Result<(), String> {
        for component in self.components.iter() {
            component.on_draw(self, renderer)?;
        }
        Ok(())
    }
}
impl Updatable for Element {
    fn update(&mut self, events: &Vec<Event>) -> Result<(), String> {
        for component in self.components.iter() {
            match component.on_update(self, events)? {
                Some(data) => self.data = data,
                _ => {}
            }
        }
        Ok(())
    }
}
