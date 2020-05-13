pub mod basic_types;
pub mod element;
pub mod elements_handler;
pub mod keyboard_mover;
pub mod renderer;
pub mod sdl_handler;
pub mod sprite_renderer;

use crate::engine::basic_types::Event;
use crate::engine::element::Element;
use crate::engine::element::ElementData;

pub trait Drawable {
    fn draw(&self, renderer: &mut dyn Renderer) -> Result<(), String>;
}

pub trait Updatable {
    fn update(&mut self, events: &Vec<Event>) -> Result<(), String>;
}

pub trait DirectMedia {
    fn clean_canvas(&mut self);
    fn draw_elements(&mut self, element: &dyn Drawable) -> Result<(), String>;
    fn init(&mut self) -> Result<(), String>;
    fn process_events(&mut self) -> Result<Vec<Event>, String>;
    fn present(&mut self);
}
pub trait Component {
    fn on_collision(&mut self) -> Result<(), String>;
    fn on_update(&self, parent: &Element, events: &Vec<Event>) -> Result<Option<ElementData>, String>;
    fn on_draw(&self, parent: &Element, renderer: &mut dyn Renderer) -> Result<(), String>;
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
