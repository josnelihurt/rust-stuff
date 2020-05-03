use crate::engine::elements_handler::ElementHandler;
use core::cell::RefCell;
use sdl2::EventPump;
use std::collections::hash_map::HashMap;
use std::rc::Rc;
use std::vec::Vec;

extern crate sdl2;
use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::{event::Event, image::InitFlag, keyboard::Keycode, render::Canvas, video::Window};

use crate::engine;
use crate::engine::basic_types::*;
use crate::engine::*;

pub struct TexturesCache<'a> {
    textures: HashMap<String, sdl2::render::Texture<'a>>,
}
impl<'a> TexturesCache<'a> {
    pub fn new() -> Self {
        TexturesCache {
            textures: HashMap::new(),
        }
    }
    pub fn load_texture(
        &mut self,
        path: String,
        creator: &'a TextureCreator<WindowContext>,
    ) -> Result<(), String> {
        let texture = creator.load_texture(path.clone())?;
        self.textures.insert(path, texture);
        Ok(())
    }
}
pub struct SdlContext {
    canvas: Canvas<Window>,
    events: EventPump,
    creator: TextureCreator<WindowContext>,
}
impl SdlContext {
    pub fn new(title: &'static str, width: u32, height: u32) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let _img = sdl2::image::init(InitFlag::PNG)?;
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
        let creator = canvas.texture_creator();
        Ok(SdlContext {
            canvas: canvas,
            events: events,
            creator: creator,
        })
    }
}
pub struct SdlHandler<'a, 'b> {
    ctx: &'a mut SdlContext,
    cache: &'a mut TexturesCache<'b>,
    listeners: Vec<Rc<RefCell<Box<dyn Mover>>>>,
}
impl<'a, 'b> SdlHandler<'a, 'b> {
    pub fn new(
        ctx: &'a mut SdlContext,
        cache: &'a mut TexturesCache<'b>,
        _fps_limit: u32,
    ) -> SdlHandler<'a, 'b> {
        SdlHandler {
            ctx: ctx,
            cache: cache,
            listeners: Vec::new(),
        }
    }
}
impl<'a, 'b> DirectMedia for SdlHandler<'a, 'b> {
    fn init(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn clean_canvas(&mut self) {
        self.ctx.canvas.clear();
    }
    fn subcribe_movement(&mut self, hnd: Rc<RefCell<Box<dyn Mover>>>) {
        self.listeners.push(hnd)
    }
    fn draw_elements(&mut self, obj: &dyn Drawable) -> Result<(), String> {
        obj.draw(self)?;
        Ok(())
    }
    fn process_events(&mut self) -> Result<(), String> {
        for event in self.ctx.events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return Err(Err::USER_EXIT.to_string());
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
    fn present(&mut self) {
        self.ctx.canvas.present();
    }
}
impl<'a, 'b> Renderer for SdlHandler<'a, 'b> {
    fn clear(&mut self) {
        let black = sdl2::pixels::Color::RGB(0, 0, 0);
        self.ctx.canvas.set_draw_color(black);
        self.ctx.canvas.clear();
    }
    fn copy(
        &mut self,
        obj: &engine::Texture,
        pos: &Vec2D,
        size: &Vec2D,
        rotation: f64,
    ) -> Result<(), String> {
        self.ctx.canvas.copy_ex(
            &obj,
            None,
            Rect::new(pos.x as i32, pos.y as i32, size.x as u32, size.y as u32),
            rotation,
            sdl2::rect::Point::new((size.x / 2.0) as i32, (size.y / 2.0) as i32),
            false,
            false,
        )
    }
    fn present(&mut self) {
        self.ctx.canvas.present();
    }
    fn texture_creator(&self) -> TextureCreator<WindowContext> {
        self.ctx.canvas.texture_creator()
    }
}

use crate::engine::basic_types::Vec2D;
use sdl2::rect::Rect;
impl Into<Option<Rect>> for Vec2D {
    fn into(self) -> Option<Rect> {
        Some(Rect::new(self.x as i32, self.y as i32, 50, 50))
    }
}
