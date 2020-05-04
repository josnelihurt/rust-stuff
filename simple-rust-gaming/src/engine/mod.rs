pub mod basic_types;
pub mod element;
pub mod elements_handler;
pub mod keyboard_mover;
pub mod renderer;
pub mod sdl_handler;
pub mod sprite_renderer;

use core::cell::RefCell;
use std::rc::Rc;
pub trait Drawable {
    fn draw(&self, renderer: &mut dyn Renderer) -> Result<(), String>;
}
pub trait Updatable {
    fn update(&mut self) -> Result<(), String>;
}
pub trait DirectMedia {
    fn clean_canvas(&mut self);
    fn draw_elements(&mut self, element: &dyn Drawable) -> Result<(), String>;
    fn init(&mut self) -> Result<(), String>;
    fn process_events(&mut self) -> Result<(), String>;
    fn subcribe_movement(&mut self, hnd: Rc<RefCell<Box<dyn Mover>>>);
    fn present(&mut self);
}
pub trait Component: Drawable + Updatable {
    fn on_collision(&mut self) -> Result<(), String>;
}
pub trait Mover {
    fn r#move(&mut self, m: basic_types::Move);
}
pub trait Renderer {
    fn clear(&mut self);
    fn copy(
        &mut self,
        texture_path: &String,
        rect: basic_types::Rect,
        rotation: f64,
    ) -> Result<(), String>;
    fn present(&mut self);
}
