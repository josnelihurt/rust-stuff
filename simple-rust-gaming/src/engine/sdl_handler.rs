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
        self.textures.insert(String::from(path), texture);
        Ok(())
    }
    pub fn get(&self, path: &String) -> Result<&sdl2::render::Texture<'a>, String> {
        match self.textures.get(path) {
            Some(t) => Ok(t),
            None => Err(err::TEXTURE_NOT_FOUND.to_string() + " " + path),
        }
    }
}
pub struct SdlContext {
    pub canvas: Canvas<Window>,
    events: EventPump,
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
        Ok(SdlContext {
            canvas: canvas,
            events: events,
        })
    }
}
pub struct SdlHandler<'a, 'b> {
    ctx: &'a mut SdlContext,
    cache: &'a mut TexturesCache<'b>,
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
    fn draw_elements(&mut self, obj: &dyn Drawable) -> Result<(), String> {
        obj.draw(self)?;
        Ok(())
    }
    fn process_events(&mut self) -> Result<Vec<basic_types::Event>, String>{
        let mut result: Vec<basic_types::Event> = Vec::new();
        for event in self.ctx.events.poll_iter() {
            match event {
                Event::Quit { .. }| Event::KeyDown {keycode: Some(Keycode::Escape),..} => return Err(err::USER_EXIT.to_string()),
                Event::KeyDown {keycode: Some(Keycode::Left),..} => result.push(basic_types::Event::Left),
                Event::KeyDown {keycode: Some(Keycode::Right),..} => result.push(basic_types::Event::Right),
                Event::KeyDown {keycode: Some(Keycode::Up),..} => result.push(basic_types::Event::Up),
                Event::KeyDown {keycode: Some(Keycode::Down),..} => result.push(basic_types::Event::Down),
                Event::KeyDown {keycode: Some(Keycode::Space),..} => result.push(basic_types::Event::Action),
                _ => {}
            }
        }
        Ok(result)
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
        texture_path: &String,
        rect: basic_types::Rect,
        rotation: f64,
    ) -> Result<(), String> {
        let texture: &sdl2::render::Texture<'a> = self.cache.get(texture_path)?;
        let rotation_point =
            sdl2::rect::Point::new((rect.width / 2) as i32, (rect.height / 2) as i32);
        self.ctx
            .canvas
            .copy_ex(texture, None, rect, rotation, rotation_point, false, false)?;
        Ok(())
    }
    fn present(&mut self) {
        self.ctx.canvas.present();
    }
}

impl Into<Option<sdl2::rect::Rect>> for basic_types::Rect {
    fn into(self) -> Option<sdl2::rect::Rect> {
        Some(sdl2::rect::Rect::new(
            self.x as i32,
            self.y as i32,
            self.width as u32,
            self.height as u32,
        ))
    }
}
