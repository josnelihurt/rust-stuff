#[allow(dead_code)]
#[allow(unused_imports)]



use std::time::Instant;

use std::error::Error;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::render::Canvas;
use sdl2::video::{Window};

struct SdlHandler {
    // sdl: sdl2::Sdl,
    events: sdl2::EventPump,
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
    Ok(SdlHandler{
        events: events,
        canvas: canvas
    })
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut sdl_hnd : SdlHandler = init_sdl("My own game", 800, 600, 60).unwrap();
    let mut event_hnd = |events : &mut sdl2::EventPump| -> bool {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { return false; },
                _ => {}
            }
        }
        return true;
    };
    'running: loop{
        if !event_hnd(&mut sdl_hnd.events) {break 'running;};
        sdl_hnd.canvas.fill_rect()
        sdl_hnd.canvas.present();
    }
    Ok(())
}
