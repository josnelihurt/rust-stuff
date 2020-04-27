
use std::rc::Rc;
use std::sync::Mutex;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct SdlHandler {
    // sdl: sdl2::Sdl,
    pub events: Rc<Mutex<sdl2::EventPump>>,
    pub canvas: Canvas<Window>,
}

pub fn init_sdl(
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