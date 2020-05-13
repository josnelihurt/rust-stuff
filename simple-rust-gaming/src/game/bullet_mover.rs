use crate::engine::basic_types::Event;
use crate::engine::element::Element;
use crate::engine::element::ElementData;
use crate::engine::Component;
use crate::engine::Renderer;

pub struct BulletMover {
    speed: f32,
}

impl Component for BulletMover {
    fn on_collision(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn on_update(&self, parent: &Element, events: &Vec<Event>) -> Result<Option<ElementData>, String> {
        let mut element = parent.data.clone();
        element.position.y = element.position.y + self.speed;
        Ok(Some(element))
    }
    fn on_draw(&self, _: &Element, _: &mut dyn Renderer) -> Result<(), String> {
        Ok(())
    }
}
impl BulletMover {
    pub fn new(speed: f32) -> Box<dyn Component> {
        Box::new(BulletMover { speed: speed })
    }
}
