use crate::engine::basic_types::Event;
use crate::engine::element::Element;
use crate::engine::element::ElementData;
use crate::engine::Component;
use crate::engine::Renderer;
pub struct PlayerShooter {}

impl Component for PlayerShooter {
    fn on_collision(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn on_update(
        &self,
        _parent: &Element,
        events: &Vec<Event>,
    ) -> Result<Option<ElementData>, String> {
        for event in events.iter() {
            //TODO: I wonder how can I avoid this clone
            if event.clone() == Event::Action {
                trace!("shoot!");
            }
        }
        Ok(None)
    }
    fn on_draw(&self, _: &Element, _: &mut dyn Renderer) -> Result<(), String> {
        Ok(())
    }
}
impl PlayerShooter {
    pub fn new() -> Box<dyn Component> {
        Box::new(PlayerShooter {})
    }
}
