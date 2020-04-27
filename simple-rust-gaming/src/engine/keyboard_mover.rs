


use std::rc::Rc;
use std::sync::Mutex;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

use crate::engine::element::Element;

pub struct KeyboardMover {
    events: Rc<Mutex<sdl2::EventPump>>,
    pub parent: Element,
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
                Event::Quit { .. } | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {std::process::exit(0)}
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