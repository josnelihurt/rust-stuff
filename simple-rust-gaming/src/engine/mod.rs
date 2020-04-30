pub mod basic_types;
pub mod element;
pub mod elements_handler;
pub mod keyboard_mover;
pub mod sdl_handler;

use std::rc::Rc;
use std::sync::Mutex;

use crate::engine::element::Element;

pub trait DirectMedia {
    fn clean_canvas(&mut self);
    fn draw_elements(&mut self, element: Rc<Mutex<Element>>);
    fn init(&mut self) -> Result<(), String>;
    fn process_events(&mut self) -> Result<(), String>;
    fn subcribe_movement(&mut self, hnd: Rc<Mutex<dyn Mover>>);
}
pub trait Component {
    fn on_update(&mut self) -> Result<bool, String>;
    fn on_draw(&mut self, renderer :&dyn Renderer) -> Result<bool, String>;
    fn on_collision(&mut self) -> Result<bool, String>;
}
pub trait Mover {
    fn r#move(&mut self, m: basic_types::Move);
}
pub trait Texture{
    fn load(&mut self, path: &'static str) -> Result<bool,String>;
}
pub trait Renderer{
    fn clean(&mut self);
    fn copy(&mut self, obj: &dyn Texture);
    fn present(&mut self);
}