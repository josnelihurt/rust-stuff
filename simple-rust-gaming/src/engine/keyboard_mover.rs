use crate::engine::element::ElementData;
use crate::engine::Component;
use crate::engine::Renderer;

use crate::engine::basic_types::*;
use crate::engine::element::Element;

pub struct KeyboardMover {
    allowed_rect: Rect,
    speed: u32,
}
impl Component for KeyboardMover {
    fn on_collision(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn on_update(
        &self,
        parent: &Element,
        events: &Vec<Event>,
    ) -> Result<Option<ElementData>, String> {
        let mut result = parent.data.clone();
        for event in events {
            match event {
                Event::Up => {
                    if parent.data.position.y > self.allowed_rect.y {
                        result.position.y = parent.data.position.y - self.speed as f32;
                    }
                }
                Event::Down => {
                    if parent.data.position.y
                        < self.allowed_rect.y + self.allowed_rect.height as f32
                    {
                        result.position.y = parent.data.position.y + self.speed as f32;
                    }
                }
                Event::Right => {
                    if parent.data.position.x < self.allowed_rect.x + self.allowed_rect.width as f32
                    {
                        result.position.x = parent.data.position.x + self.speed as f32;
                    }
                }
                Event::Left => {
                    if parent.data.position.x > self.allowed_rect.x {
                        result.position.x = parent.data.position.x - self.speed as f32;
                    }
                }
                _ => {}
            };
        }
        Ok(Some(result))
    }
    fn on_draw(&self, _parent: &Element, _renderer: &mut dyn Renderer) -> Result<(), String> {
        Ok(())
    }
}

impl KeyboardMover {
    pub fn new(allowed_rect: Rect, speed: u32) -> Box<dyn Component> {
        Box::new(KeyboardMover {
            allowed_rect: allowed_rect,
            speed: speed,
        })
    }
}
