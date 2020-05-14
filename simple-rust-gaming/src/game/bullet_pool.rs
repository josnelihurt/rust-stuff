use super::bullet;
use crate::engine::element::Element;

pub fn new() -> Vec<Element> {
    let mut result = Vec::new();
    for i in 1..10 {
        let mut bullet = bullet::new();
        bullet.data.active = false;
        result.push(bullet);
    }
    result
}
