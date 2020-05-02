pub mod basic_types;
pub mod element;
pub mod elements_handler;
pub mod keyboard_mover;
pub mod renderer;
pub mod sdl_handler;
pub mod sprite_renderer;

use core::cell::RefCell;
use crate::engine::basic_types::Vec2D;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use std::rc::Rc;


use crate::engine::element::Element;

pub trait DirectMedia {
    fn clean_canvas(&mut self);
    fn draw_elements(&mut self, element: Rc<RefCell<Element>>);
    fn init(&mut self) -> Result<(), String>;
    fn process_events(&mut self) -> Result<(), String>;
    fn subcribe_movement(&mut self, hnd: Rc<RefCell<Box<dyn Mover>>>);
}
pub trait Component {
    fn on_update(&mut self) -> Result<bool, String>;
    fn on_draw(&self, renderer: &mut dyn Renderer) -> Result<bool, String>;
    fn on_collision(&mut self) -> Result<bool, String>;
}
pub trait Mover {
    fn r#move(&mut self, m: basic_types::Move);
}
pub type Texture<'a> = sdl2::render::Texture<'a>;
pub trait Renderer {
    fn clear(&mut self);
    fn copy(&mut self, obj: &Texture, pos: &Vec2D, size: &Vec2D);
    fn present(&mut self);
    fn texture_creator(&self) -> TextureCreator<WindowContext>;
}
