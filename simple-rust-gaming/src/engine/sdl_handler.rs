extern crate sdl2;

use sdl2::{event::Event, image::InitFlag, keyboard::Keycode, render::Canvas, video::Window};
use std::rc::Rc;
use std::sync::Mutex;
use std::vec::Vec;

use crate::engine::basic_types::Move;
use crate::engine::{element::Element, DirectMedia, Mover};

pub struct SdlHandler {
    canvas: Canvas<Window>,
    events: sdl2::EventPump,
    fps_limit: u32,
    height: u32,
    listeners: Vec<Rc<Mutex<dyn Mover>>>,
    title: String,
    width: u32,
}
impl SdlHandler {
    pub fn new(title: &'static str, width: u32, height: u32, fps_limit: u32) -> SdlHandler {
        let sdl = sdl2::init().unwrap();
        let vid_s = sdl.video().unwrap();
        let events = sdl.event_pump().unwrap();

        let window = vid_s
            .window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        SdlHandler {
            canvas: canvas,
            events: events,
            fps_limit: fps_limit,
            height: height,
            listeners: Vec::new(),
            title: String::from(title),
            width: width,
        }
    }
    fn texture_from_file(&mut self) -> i32 {
        let _img = sdl2::image::init(InitFlag::PNG).unwrap();
        let texture_creator = self.canvas.texture_creator();
        //self.canvas.copy_ex(texture: &Texture, src: R1, dst: R2, angle: f64, center: P, flip_horizontal: bool, flip_vertical: bool)
        0
    }
}
impl DirectMedia for SdlHandler {
    fn init(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn clean_canvas(&mut self) {
        let black = sdl2::pixels::Color::RGB(0, 0, 0);
        self.canvas.set_draw_color(black);
        self.canvas.clear();
    }
    fn subcribe_movement(&mut self, hnd: Rc<Mutex<dyn Mover>>) {
        self.listeners.push(hnd)
    }
    fn draw_elements(&mut self, element: Rc<Mutex<Element>>) {
        let white = sdl2::pixels::Color::RGB(255, 255, 255);
        self.canvas.set_draw_color(white);
        self.canvas
            .fill_rect(element.lock().unwrap().position.clone())
            .unwrap();
        self.texture_from_file();
        self.canvas.present();
    }
    fn process_events(&mut self) -> Result<(), String> {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return Err("Exit from user".to_string());
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let m = Move::Left;
                    for listener in self.listeners.iter_mut() {
                        listener.lock().unwrap().r#move(m.clone());
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let m = Move::Right;
                    for listener in self.listeners.iter_mut() {
                        listener.lock().unwrap().r#move(m.clone());
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    let m = Move::Up;
                    for listener in self.listeners.iter_mut() {
                        listener.lock().unwrap().r#move(m.clone());
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let m = Move::Down;
                    for listener in self.listeners.iter_mut() {
                        listener.lock().unwrap().r#move(m.clone());
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

use crate::engine::basic_types::Vec2D;
use sdl2::rect::Rect;
impl Into<Option<Rect>> for Vec2D {
    fn into(self) -> Option<Rect> {
        Some(Rect::new(self.x as i32, self.y as i32, 10, 10))
    }
}
