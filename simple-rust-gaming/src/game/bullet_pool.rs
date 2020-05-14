use super::bullet;
use crate::engine::element::Element;

pub struct BulletPool<'a> {
    bullets: Vec<&'a Element>,
}
impl<'a> BulletPool<'a> {
    pub fn new(bullets: &'a Vec<Element>) -> BulletPool<'a> {
        let mut bullets_ref = Vec::new();
        bullets.iter().for_each(|x| bullets_ref.push(x));
        BulletPool {
            bullets: bullets_ref,
        }
    }
}

pub fn new() -> Vec<Element> {
    let mut result = Vec::new();
    for _ in 1..10 {
        let mut bullet = bullet::new();
        bullet.data.active = false;
        result.push(bullet);
    }
    result
}
