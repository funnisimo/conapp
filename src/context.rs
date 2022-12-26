use crate::{Image, RGBA};

use super::input::{AppInput, InputApi};
use super::Font;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uni_gl::{BufferBit, WebGLRenderingContext};

pub(crate) static SUBCELL_BYTES: &[u8] = include_bytes!("../resources/subcell.png");

/// This is the complete API provided to you in [`crate::Screen`] methods.
pub trait AppContext {
    /// Clears the screen, with an optional color
    fn clear(&self, color: Option<RGBA>);
    /// return the input [`InputApi`] to check user mouse and keyboard input
    fn input(&self) -> &dyn InputApi;
    /// return the current framerate
    fn fps(&self) -> u32;
    /// return the average framerate since the start of the game
    fn average_fps(&self) -> u32;
    /// the time in ms since the last update call
    fn frame_time_ms(&self) -> f32;

    /// return the current screen size
    fn get_screen_size(&self) -> (u32, u32);

    /// return the [`WebGLRenderingContext`]
    fn gl(&self) -> &WebGLRenderingContext;

    /// Load a font program for the given font file
    fn load_font(&mut self, fontpath: &str) -> Rc<RefCell<Font>>;

    /// Load an image file
    fn load_image(&mut self, imgpath: &str) -> Rc<RefCell<Image>>;
}

pub(crate) struct AppContextImpl {
    // pub(super) cons: Vec<Console>,
    pub input: AppInput,
    pub fps: u32,
    pub average_fps: u32,
    pub screen_size: (u32, u32),
    pub frame_time_ms: f32,
    pub gl: WebGLRenderingContext,
    pub fonts: HashMap<String, Rc<RefCell<Font>>>,
    pub images: HashMap<String, Rc<RefCell<Image>>>,
    pub ready: bool,
}

impl AppContext for AppContextImpl {
    fn gl(&self) -> &WebGLRenderingContext {
        &self.gl
    }

    fn clear(&self, color: Option<RGBA>) {
        match color {
            None => self.gl.clear(BufferBit::Color),
            Some(c) => {
                let data = c.to_f32();
                self.gl.clear_color(data.0, data.1, data.2, data.3);
            }
        }
    }

    fn input(&self) -> &dyn InputApi {
        &self.input
    }
    fn fps(&self) -> u32 {
        self.fps
    }
    fn average_fps(&self) -> u32 {
        self.average_fps
    }

    fn frame_time_ms(&self) -> f32 {
        self.frame_time_ms
    }

    fn get_screen_size(&self) -> (u32, u32) {
        self.screen_size
    }

    fn load_font(&mut self, fontpath: &str) -> Rc<RefCell<Font>> {
        if let Some(font) = self.fonts.get(fontpath) {
            return font.clone();
        }

        let font = Font::new(fontpath, &self.gl);
        self.fonts.insert(fontpath.to_owned(), font.clone());
        self.ready = false;
        font
    }

    fn load_image(&mut self, imgpath: &str) -> Rc<RefCell<Image>> {
        if let Some(image) = self.images.get(imgpath) {
            return image.clone();
        }

        let image = Image::new(imgpath);
        self.images.insert(imgpath.to_owned(), image.clone());
        self.ready = false;
        image
    }

    // fn get_font(&self, fontpath: &str) -> Option<Rc<RefCell<Font>>> {
    //     match self.fonts.get(fontpath) {
    //         None => None,
    //         Some(font) => Some(font.clone()),
    //     }
    // }
}

impl AppContextImpl {
    pub fn new(gl: WebGLRenderingContext, screen_size: (u32, u32), input: AppInput) -> Self {
        let sub_cell_font: Rc<RefCell<Font>> = Font::from_bytes(SUBCELL_BYTES, &gl);
        let mut fonts = HashMap::new();
        fonts.insert("SUBCELL".to_owned(), sub_cell_font);

        AppContextImpl {
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

    pub fn resize(&mut self, screen_width: u32, screen_height: u32) {
        self.screen_size = (screen_width, screen_height);
    }

    pub fn load_files(&mut self) -> bool {
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
}
