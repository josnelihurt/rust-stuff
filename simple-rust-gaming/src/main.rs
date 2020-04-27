#[allow(dead_code)]
#[allow(unused_imports)]

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::error::Error;
use std::rc::Rc;
use std::sync::Mutex;
use std::time::Instant;
mod engine;
use engine::element::Element;
use engine::sdl_handler::SdlHandler;
use engine::sdl_handler;




pub struct KeyboardMover {
    events: Rc<Mutex<sdl2::EventPump>>,
    parent: Element,
}
impl KeyboardMover {
    pub fn new(events: Rc<Mutex<sdl2::EventPump>>, parent: Element) -> KeyboardMover {
        KeyboardMover {
            events: events,
            parent: parent,
        }
    }
    pub fn on_update(&mut self) {
        let mut events = self.events.lock().unwrap();
        for event in events.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    self.parent.r#move(-10, 0);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    self.parent.r#move(10, 0);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    self.parent.r#move(0, -10);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    self.parent.r#move(0, 10);
                }
                _ => {}
            }
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting process");
    let mut sdl_hnd: SdlHandler = sdl_handler::init_sdl("My own game", 800, 600, 60).unwrap();
    let mut event_hnd = |events: Rc<Mutex<sdl2::EventPump>>| -> bool {
        let mut events = events.lock().unwrap();
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    println!("ESCAPE!");
                    return false;
                }
                _ => {}
            }
        }
        return true;
    };
    let black = sdl2::pixels::Color::RGB(0, 0, 0);
    let white = sdl2::pixels::Color::RGB(255, 255, 255);
    let player = Element::new(10, 10, 10, 10);
    let mut playerMover = KeyboardMover::new(sdl_hnd.events.clone(), player);
    'running: loop {
        sdl_hnd.canvas.set_draw_color(black);
        sdl_hnd.canvas.clear();
        sdl_hnd.canvas.set_draw_color(white);
        playerMover.on_update();
        if !event_hnd(sdl_hnd.events.clone()) {
            break 'running;
        };
        sdl_hnd.canvas.fill_rect(playerMover.parent.rect);
        sdl_hnd.canvas.present();
    }

    println!("Exiting process");
    Ok(())
}
