extern crate num;
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
