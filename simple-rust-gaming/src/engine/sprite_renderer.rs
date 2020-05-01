use crate::engine::basic_types::Vec2D;
use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;
use std::rc::Rc;

use crate::engine::element::Element;
use crate::engine::{Component, Renderer, Texture};

pub struct SpriteRenderer<'a> {
    parent: &'a Element,
}
impl<'a> SpriteRenderer<'a> {
    pub fn new(parent: &'a Element) -> Self {
        SpriteRenderer { parent: parent }
    }
}
impl Component for SpriteRenderer<'_> {
    fn on_update(&mut self) -> Result<bool, String> {
        Ok(true)
    }
    fn on_draw(&mut self, renderer: &mut Renderer) -> Result<bool, String> {
        let texture_creator: TextureCreator<_> = renderer.texture_creator();
        let texture = texture_creator
            .load_texture("res/sprites/player.png")
            .unwrap();
        renderer.copy(&texture, &self.parent.position, &Vec2D::new(50, 50));
        Ok(true)
    }
    fn on_collision(&mut self) -> Result<bool, String> {
        Ok(true)
    }
}
