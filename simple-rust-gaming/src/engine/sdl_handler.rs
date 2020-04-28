use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::rc::Rc;
use std::sync::Mutex;
use std::vec::Vec;

use crate::engine::Mover;
use crate::engine::basic_types::Move;

pub struct SdlHandler {
    // sdl: sdl2::Sdl,
    pub events: Rc<Mutex<sdl2::EventPump>>,
    pub canvas: Canvas<Window>,
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
            events: Rc::new(Mutex::new(events)),
            canvas: canvas,
            listeners: Vec::new(),
        })
    }
    pub fn subcribe_movement(&mut self, hnd: &dyn Mover){
        //self.listeners.push(Box::new(hnd))
    }
    pub fn process_events(&mut self){
        let mut events = self.events.lock().unwrap();
        for event in events.poll_iter() {
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
