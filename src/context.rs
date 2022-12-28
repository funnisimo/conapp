use crate::{Image, RGBA};

use super::input::{AppInput, InputApi};
use super::Font;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uni_gl::{BufferBit, WebGLRenderingContext};

pub(crate) static SUBCELL_BYTES: &[u8] = include_bytes!("../resources/subcell.png");

pub struct AppContext {
    // pub(super) cons: Vec<Console>,
    pub(crate) input: AppInput,
    pub(crate) fps: u32,
    pub(crate) average_fps: u32,
    pub(crate) screen_size: (u32, u32),
    pub(crate) frame_time_ms: f64,
    pub(crate) gl: WebGLRenderingContext,
    pub(crate) fonts: HashMap<String, Rc<RefCell<Font>>>,
    pub(crate) images: HashMap<String, Rc<RefCell<Image>>>,
    pub(crate) ready: bool,
}

impl AppContext {
    pub(crate) fn new(gl: WebGLRenderingContext, screen_size: (u32, u32), input: AppInput) -> Self {
        let sub_cell_font: Rc<RefCell<Font>> = Font::from_bytes(SUBCELL_BYTES, &gl);
        let mut fonts = HashMap::new();
        fonts.insert("SUBCELL".to_owned(), sub_cell_font);

        AppContext {
            input,
            fps: 0,
            average_fps: 0,
            screen_size: screen_size,
            frame_time_ms: 0.0,
            gl,
            fonts,
            images: HashMap::new(),
            ready: false,
        }
    }

    pub(crate) fn resize(&mut self, screen_width: u32, screen_height: u32) {
        self.screen_size = (screen_width, screen_height);
    }

    pub(crate) fn load_files(&mut self) -> bool {
        if self.ready {
            return true;
        }
        let mut ready = true;
        for (_, font) in self.fonts.iter_mut() {
            if !font.borrow_mut().load_async(&self.gl) {
                ready = false;
            }
        }
        for (_, image) in self.images.iter_mut() {
            if !image.borrow_mut().load_async() {
                ready = false;
            }
        }
        if ready {
            self.ready = true;
        }
        ready
    }

    pub fn gl(&self) -> &WebGLRenderingContext {
        &self.gl
    }

    pub fn clear(&self, color: Option<RGBA>) {
        match color {
            None => self.gl.clear(BufferBit::Color),
            Some(c) => {
                let data = c.to_f32();
                self.gl.clear_color(data.0, data.1, data.2, data.3);
            }
        }
    }

    pub fn input(&self) -> &dyn InputApi {
        &self.input
    }

    pub fn fps(&self) -> u32 {
        self.fps
    }

    pub fn average_fps(&self) -> u32 {
        self.average_fps
    }

    pub fn frame_time_ms(&self) -> f64 {
        self.frame_time_ms
    }

    pub fn get_screen_size(&self) -> (u32, u32) {
        self.screen_size
    }

    pub fn get_font(&mut self, fontpath: &str) -> Rc<RefCell<Font>> {
        if let Some(font) = self.fonts.get(fontpath) {
            return font.clone();
        }

        let font = Font::new(fontpath, &self.gl);
        self.fonts.insert(fontpath.to_owned(), font.clone());
        self.ready = false;
        font
    }

    pub fn get_image(&mut self, imgpath: &str) -> Rc<RefCell<Image>> {
        if let Some(image) = self.images.get(imgpath) {
            return image.clone();
        }

        let image = Image::new(imgpath);
        self.images.insert(imgpath.to_owned(), image.clone());
        self.ready = false;
        image
    }
}
