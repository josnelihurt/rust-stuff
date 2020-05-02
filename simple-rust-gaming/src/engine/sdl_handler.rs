extern crate sdl2;

use core::cell::RefCell;

use sdl2::render::{TextureCreator};
use sdl2::video::WindowContext;
use sdl2::{event::Event, image::InitFlag, keyboard::Keycode, render::Canvas, video::Window};

use std::rc::Rc;

use std::vec::Vec;

use crate::engine;
use crate::engine::basic_types::Move;
use crate::engine::{element::Element, DirectMedia, Mover, Renderer};
// struct SdlTexture {
//     path: &'static str,
// }

// impl SdlTexture {
//     pub fn new(path: &'static str, creator: &dyn sdl2::image::LoadTexture) -> SdlTexture {
//         SdlTexture { path: path }
//     }
//}

struct SdlRenderer {
    canvas: Canvas<Window>,
}
impl SdlRenderer {
    fn new(canvas: Canvas<Window>) -> SdlRenderer {
        let _img = sdl2::image::init(InitFlag::PNG).unwrap();
        SdlRenderer { canvas: canvas }
    }
}
impl Renderer for SdlRenderer {
    fn clear(&mut self) {
        let black = sdl2::pixels::Color::RGB(0, 0, 0);
        self.canvas.set_draw_color(black);
        self.canvas.clear();
    }
    fn copy(&mut self, obj: &engine::Texture, pos: &Vec2D, size: &Vec2D) {
        self.canvas.copy(
            &obj,
            None,
            Rect::new(pos.x as i32, pos.y as i32, size.x as u32, size.y as u32),
        );
    }
    fn present(&mut self) {
        self.canvas.present();
    }
    fn texture_creator(&self) -> TextureCreator<WindowContext> {
        self.canvas.texture_creator()
    }
}
pub struct SdlHandler {
    canvas: SdlRenderer,
    events: sdl2::EventPump,
    fps_limit: u32,
    height: u32,
    listeners: Vec<Rc<RefCell<Box<dyn Mover>>>>,
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
            canvas: SdlRenderer::new(canvas),
            events: events,
            fps_limit: fps_limit,
            height: height,
            listeners: Vec::new(),
            title: String::from(title),
            width: width,
        }
    }
}
impl DirectMedia for SdlHandler {
    fn init(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn clean_canvas(&mut self) {
        self.canvas.clear();
    }
    fn subcribe_movement(&mut self, hnd: Rc<RefCell<Box<dyn Mover>>>) {
        self.listeners.push(hnd)
    }
    fn draw_elements(&mut self, element: Rc<RefCell<Element>>) {
        //element.lock().unwrap().draw(&mut self.canvas);
        element.borrow().draw(&mut self.canvas).unwrap();
        // for item in components.iter_mut() {
        //     //item.lock().unwrap().on_update();
        //     item.lock().unwrap().on_draw(&mut self.canvas).unwrap();
        // }
        // let texture_creator: TextureCreator<_> = self.canvas.canvas.texture_creator();
        // let texture = texture_creator
        //     .load_texture("res/sprites/player.png")
        //     .unwrap();

        // self.canvas
        //     .canvas
        //     .copy(&texture, None, element.lock().unwrap().position.clone());
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
                    for listener in self.listeners.iter() {
                        listener.borrow_mut().r#move(m.clone());
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let m = Move::Right;
                    for listener in self.listeners.iter_mut() {
                        listener.borrow_mut().r#move(m.clone());
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    let m = Move::Up;
                    for listener in self.listeners.iter_mut() {
                        listener.borrow_mut().r#move(m.clone());
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let m = Move::Down;
                    for listener in self.listeners.iter_mut() {
                        listener.borrow_mut().r#move(m.clone());
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
        Some(Rect::new(self.x as i32, self.y as i32, 50, 50))
    }
}
