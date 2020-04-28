pub mod basic_types;
pub mod element;
pub mod keyboard_mover;
pub mod sdl_handler;

pub trait Component {
    fn on_update() -> Result<bool, String>;
    fn on_draw() -> Result<bool, String>;
    fn on_collision() -> Result<bool, String>;
}
