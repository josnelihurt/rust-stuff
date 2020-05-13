extern crate num;

pub mod err {
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
#[derive(Clone, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: u32,
    pub height: u32,
}
impl Rect {
    pub fn new<T: num::cast::AsPrimitive<f32>, U: num::cast::AsPrimitive<u32>>(
        x: T,
        y: T,
        width: U,
        height: U,
    ) -> Rect {
        Rect {
            x: x.as_(),
            y: y.as_(),
            width: width.as_(),
            height: height.as_(),
        }
    }
}
#[derive(Clone)]
pub enum Event {
    Up,
    Down,
    Right,
    Left,
    Action,
}
