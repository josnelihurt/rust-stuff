use sdl2::rect::Rect;
pub struct Element {
    pub rect: sdl2::rect::Rect,
}
impl Element {
    pub fn new(x: i32, y: i32, size_x: u32, size_y: u32) -> Element {
        Element {
            rect: Rect::new(x, y, size_x, size_y),
        }
    }
    pub fn r#move(&mut self, dx: i32, dy: i32) {
        self.rect.x += dx;
        self.rect.y += dy;
    }
}
