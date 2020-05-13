use crate::engine::basic_types::Event;
use crate::engine::element::Element;
use crate::engine::element::ElementData;
use crate::engine::Component;
use crate::engine::Renderer;
pub struct PlayerShooter {
    pending_action: Option<()>
}

impl Component for PlayerShooter {
    fn on_collision(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn on_update(&self, parent: &Element, events: &Vec<Event>) -> Result<Option<ElementData>, String>{
        if let Some(()) = self.pending_action {
            trace!("shoot!");
            //self.listener.pending_action = None;
        }
        Ok(None)
    }
    fn on_draw(&self, _: &Element, _: &mut dyn Renderer) -> Result<(), String> {
        Ok(())
    }
}
impl PlayerShooter {
    pub fn new() -> Box<dyn Component>  {
        Box::new(PlayerShooter {
            pending_action: None,
        })
    }
}
