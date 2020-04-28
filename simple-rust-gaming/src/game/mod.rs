extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::rc::Rc;
use std::sync::Mutex;
use std::time::Instant;

use crate::engine::element::Element;
use crate::engine::keyboard_mover::KeyboardMover;
use crate::engine::sdl_handler;
use crate::engine::sdl_handler::SdlHandler;

pub struct GameLogic {}
pub trait Game {
    fn new() -> Self;
    fn run(&self);
}
impl Game for GameLogic {
    fn new() -> GameLogic {
        GameLogic {}
    }
    fn run(&self) {
        let mut sdl_hnd: SdlHandler = sdl_handler::init_sdl("My own game", 800, 600, 60).unwrap();
        let black = sdl2::pixels::Color::RGB(0, 0, 0);
        let white = sdl2::pixels::Color::RGB(255, 255, 255);
        let player = Element::new(10, 10, 10, 10);
        let mut player_mover = KeyboardMover::new(sdl_hnd.events.clone(), player);
        '_running: loop {
            sdl_hnd.canvas.set_draw_color(black);
            sdl_hnd.canvas.clear();
            sdl_hnd.canvas.set_draw_color(white);
            player_mover.on_update();
            // if !event_hnd(sdl_hnd.events.clone()) {
            //     break 'running;
            // };
            sdl_hnd
                .canvas
                .fill_rect(player_mover.parent.position.clone());
            sdl_hnd.canvas.present();
        }
    }
}
