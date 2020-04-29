pub mod basic_types;
pub mod element;
pub mod elements_handler;
pub mod keyboard_mover;
pub mod sdl_handler;

pub trait Component {
    fn on_update(&mut self) -> Result<bool, String>;
    fn on_draw(&mut self) -> Result<bool, String>;
    fn on_collision(&mut self) -> Result<bool, String>;
}
pub trait Mover {
    fn r#move(&mut self, m: basic_types::Move);
}
