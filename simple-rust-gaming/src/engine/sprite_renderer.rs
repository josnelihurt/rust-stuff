use core::cell::RefCell;
use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;
use std::rc::Rc;

use crate::engine::basic_types::*;
use crate::engine::{element::Element, Component, Renderer};

pub struct SpriteRenderer {
    parent: Rc<RefCell<Element>>,
}
impl SpriteRenderer {
    pub fn new(parent: Rc<RefCell<Element>>) -> RefCell<Box<dyn Component>> {
        RefCell::new(Box::new(SpriteRenderer { parent: parent }))
    }
}
impl Component for SpriteRenderer {
    fn on_update(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn on_draw(&self, renderer: &mut dyn Renderer) -> Result<(), String> {
        let texture_creator: TextureCreator<_> = renderer.texture_creator();
        let texture = texture_creator
            .load_texture("res/sprites/player.png")
            .unwrap();
        let position = self.parent.borrow().position.clone();
        renderer.copy(&texture, &position, &Vec2D::new(50, 50));
        Ok(())
    }
    fn on_collision(&mut self) -> Result<(), String> {
        Ok(())
    }
}
