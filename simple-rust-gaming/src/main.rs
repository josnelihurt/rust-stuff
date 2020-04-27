#[allow(dead_code)]
#[allow(unused_imports)]
struct s {}
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

struct SdlHandler {
    // sdl: sdl2::Sdl,
    events: Rc<Mutex<sdl2::EventPump>>,
    canvas: Canvas<Window>,
}

fn init_sdl(
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
    })
}

pub struct KeyboardMover {
    events: Rc<Mutex<sdl2::EventPump>>,
    parent: engine::Element,
}
impl KeyboardMover {
    pub fn new(events: Rc<Mutex<sdl2::EventPump>>, parent: engine::Element) -> KeyboardMover {
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
    let mut sdl_hnd: SdlHandler = init_sdl("My own game", 800, 600, 60).unwrap();
    let mut event_hnd = |events: Rc<Mutex<sdl2::EventPump>>| -> bool {
        let mut events = events.lock().unwrap();
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return false;
                }
                _ => {}
            }
        }
        return true;
    };
    let black = sdl2::pixels::Color::RGB(0, 0, 0);
    let white = sdl2::pixels::Color::RGB(255, 255, 255);
    let player = engine::Element::new(10, 10, 10, 10);
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
    Ok(())
}
