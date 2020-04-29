use crate::engine::basic_types::Vec2D;
use crate::engine::Component;

pub struct Element {
    pub active: bool,
    pub position: Vec2D,
    pub rotation: f32,
    pub components: Vec<Box<dyn Component>>,
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
    pub fn r#move<T: num::cast::AsPrimitive<f32>>(&mut self, dx: T, dy: T) {
        self.position.x += dx.as_();
        self.position.y += dy.as_();
    }
    #[warn(unused_must_use)]
    pub fn draw(&mut self) -> Result<bool, String> {
        for item in self.components.iter_mut() {
            item.on_draw()?;
        }
        Ok(true)
    }
    #[warn(unused_must_use)]
    pub fn update(&mut self) -> Result<bool, String> {
        for item in self.components.iter_mut() {
            item.on_update()?;
        }
        Ok(true)
    }
}
