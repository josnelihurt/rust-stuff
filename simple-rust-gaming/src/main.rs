#[allow(dead_code)]
#[allow(unused_imports)]
use std::time::Instant;

use std::error::Error;

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
    Ok(true)
}

fn game_loop() {}

fn main() -> Result<(), Box<dyn Error>> {
    init_sdl("My own game", 800, 600, 60)?;
    Ok(())
}
