extern crate num;

pub mod Err {
    pub const USER_EXIT: &'static str = "Exit from user";
    pub const TEXTURE_NOT_FOUND: &'static str = "Unable to find texture";
}

#[derive(Clone)]
pub struct Vec2D {
    pub x: f32,
    pub y: f32,
}

impl Vec2D {
    pub fn new<T: num::cast::AsPrimitive<f32>>(x: T, y: T) -> Vec2D {
        Vec2D {
            x: x.as_(),
            y: y.as_(),
        }
    }
}
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
#[derive(Clone)]
pub enum Move {
    Up,
    Down,
    Right,
    Left,
}
