use crate::config;
use crate::engine::{element::Element, sprite_renderer::SpriteRenderer, DirectMedia};

pub mod texture {
    pub const PATH: &'static str = "res/sprites/basic_enemy.png";
}
pub fn new(_dm: &mut dyn DirectMedia) -> Element {
    let size: u32 = 50;
    let initial_x: i32 = (config::screen::WIDTH / 2) - (size / 2) as i32;
    let initial_y: i32 = 50;
    let mut element = Element::new(initial_x, initial_y, size, size, 0.0);
    let renderer = SpriteRenderer::new(String::from(texture::PATH));
    element.add_component(renderer);
    element
}
