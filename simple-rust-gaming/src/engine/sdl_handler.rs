extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::rc::Rc;
use std::sync::Mutex;
use std::vec::Vec;

use crate::engine::basic_types::Move;
use crate::engine::element::Element;
use crate::engine::Mover;

pub struct SdlHandler {
    // sdl: sdl2::Sdl,
    events: sdl2::EventPump,
    canvas: Canvas<Window>,
    pub listeners: Vec<Rc<Mutex<dyn Mover>>>,
}
impl SdlHandler {
    pub fn new(
        title: &'static str,
        width: u32,
        height: u32,
        fps_limit: u32,
    ) -> Result<SdlHandler, String> {
        let sdl = sdl2::init()?;
        let vid_s = sdl.video()?;
        let events = sdl.event_pump()?;

        let window = vid_s
            .window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;
        Ok(SdlHandler {
            events: events,
            canvas: canvas,
            listeners: Vec::new(),
        })
    }
    pub fn clean_canvas(&mut self) {
        let black = sdl2::pixels::Color::RGB(0, 0, 0);
        self.canvas.set_draw_color(black);
        self.canvas.clear();
    }
    pub fn subcribe_movement(&mut self, hnd: &dyn Mover) {
        //self.listeners.push(Box::new(hnd))
    }
    pub fn draw_elements(&mut self, element: Rc<Mutex<Element>>) {
        let white = sdl2::pixels::Color::RGB(255, 255, 255);
        self.canvas.set_draw_color(white);
        self.canvas
            .fill_rect(element.lock().unwrap().position.clone())
            .unwrap();
        self.canvas.present();
    }
    pub fn process_events(&mut self) {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    self.listeners[0].lock().unwrap().r#move(Move::Left);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    self.listeners[0].lock().unwrap().r#move(Move::Right);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    self.listeners[0].lock().unwrap().r#move(Move::Up);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    self.listeners[0].lock().unwrap().r#move(Move::Down);
                }
                _ => {}
            }
        }
    }
}

use crate::engine::basic_types::Vec2D;
use sdl2::rect::Rect;
impl Into<Option<Rect>> for Vec2D {
    fn into(self) -> Option<Rect> {
        Some(Rect::new(self.x as i32, self.y as i32, 10, 10))
    }
}
