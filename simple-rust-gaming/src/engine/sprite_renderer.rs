

use crate::engine::{element::Element, *};

pub struct SpriteRenderer {
    path: String, // Idk how to store the sprite, now I am using a cache :/
}
impl SpriteRenderer {
    pub fn new(path: String) -> Box<dyn Component> {
        Box::new(SpriteRenderer { path: path })
    }
}
impl Component for SpriteRenderer {
    fn on_collision(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn on_update(&self, _parent: &Element, _events: &Vec<Event>) -> Result<Option<ElementData>, String> {
        Ok(None)
    }
    fn on_draw(&self, parent: &Element, renderer: &mut dyn Renderer) -> Result<(), String> {
        let rect = parent.data.position.clone();
        let rot = parent.data.rotation;
        renderer.copy(&self.path, rect, rot)?;
        Ok(())
    }
}
