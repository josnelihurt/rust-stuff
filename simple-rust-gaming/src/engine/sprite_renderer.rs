use core::cell::RefCell;
use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;
use std::rc::Rc;

use crate::engine::basic_types::*;
use crate::engine::{element::Element, Component, Renderer};

pub struct SpriteRenderer {
    parent: Rc<RefCell<Element>>,
    path: String, // Idn how to store the sprite :/
}
impl SpriteRenderer {
    pub fn new(parent: Rc<RefCell<Element>>, path: String) -> RefCell<Box<dyn Component>> {
        RefCell::new(Box::new(SpriteRenderer {
            parent: parent,
            path: path,
        }))
    }
}
impl Component for SpriteRenderer {
    fn on_update(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn on_draw(&self, renderer: &mut dyn Renderer) -> Result<(), String> {
        let texture_creator: TextureCreator<_> = renderer.texture_creator();
        let texture = texture_creator.load_texture(&self.path)?;
        let position = self.parent.borrow().position.clone();
        let rotation = self.parent.borrow().rotation;
        renderer.copy(&texture, &position, &Vec2D::new(50, 50), rotation)?;
        Ok(())
    }
    fn on_collision(&mut self) -> Result<(), String> {
        Ok(())
    }
}
