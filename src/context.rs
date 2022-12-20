use crate::RGBA;

use super::input::{AppInput, InputApi};
use super::Font;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uni_gl::{BufferBit, WebGLRenderingContext};

/// This is the complete doryen-rs API provided to you by [`App`] in [`Engine::update`] and [`Engine::render`] methods.
pub trait AppContext {
    /// return the root console that you can use to draw things on the screen
    // fn con(&self) -> &Console;

    /// return the root console that you can use to draw things on the screen
    // fn con_mut(&mut self) -> &mut Console;

    // /// gets you access to the additional consoles that you add to the app
    // fn get_console(&self, idx: usize) -> Option<&Console>;

    // /// gets you access to the additional consoles that you add to the app
    // fn get_console_mut(&mut self, idx: usize) -> Option<&mut Console>;
    fn clear(&self, color: Option<RGBA>);

    /// return the input API to check user mouse and keyboard input
    fn input(&self) -> &dyn InputApi;
    /// return the current framerate
    fn fps(&self) -> u32;
    /// return the average framerate since the start of the game
    fn average_fps(&self) -> u32;
    /// the time in ms since the last update call
    fn frame_time_ms(&self) -> f32;

    /// return the current screen size
    fn get_screen_size(&self) -> (u32, u32);
    // fn clear_all(&mut self) -> ();

    fn gl(&self) -> &WebGLRenderingContext;

    fn load_font(&mut self, fontpath: &str) -> Rc<RefCell<Font>>;
    fn get_font(&self, fontpath: &str) -> Option<Rc<RefCell<Font>>>;
}

pub struct ContextImpl {
    // pub(super) cons: Vec<Console>,
    pub(crate) input: AppInput,
    pub(crate) fps: u32,
    pub(crate) average_fps: u32,
    pub(crate) screen_size: (u32, u32),
    pub(crate) frame_time_ms: f32,
    pub(crate) gl: WebGLRenderingContext,
    pub(crate) fonts: HashMap<String, Rc<RefCell<Font>>>,
    pub(crate) ready: bool,
}

impl AppContext for ContextImpl {
    // fn con(&self) -> &Console {
    //     &self.cons[0]
    // }

    // fn con_mut(&mut self) -> &mut Console {
    //     &mut self.cons[0]
    // }

    // fn get_console(&self, idx: usize) -> Option<&Console> {
    //     self.cons.get(idx)
    // }

    // fn get_console_mut(&mut self, idx: usize) -> Option<&mut Console> {
    //     self.cons.get_mut(idx)
    // }

    fn gl(&self) -> &WebGLRenderingContext {
        &self.gl
    }

    fn clear(&self, color: Option<RGBA>) {
        match color {
            None => self.gl.clear(BufferBit::Color),
            Some(c) => {
                let data = c.as_f32();
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

        let font = Rc::new(RefCell::new(Font::new(fontpath, self)));
        self.fonts.insert(fontpath.to_owned(), font.clone());
        self.ready = false;
        font
    }

    fn get_font(&self, fontpath: &str) -> Option<Rc<RefCell<Font>>> {
        match self.fonts.get(fontpath) {
            None => None,
            Some(font) => Some(font.clone()),
        }
    }
}

impl ContextImpl {
    pub fn resize(&mut self, screen_width: u32, screen_height: u32) {
        self.screen_size = (screen_width, screen_height);
        // for con in self.cons.iter_mut() {
        //     con.screen_resize(gl, screen_width, screen_height);
        // }
    }

    pub fn load_fonts(&mut self) -> bool {
        if self.ready {
            return true;
        }
        let mut ready = true;
        for (_, font) in self.fonts.iter_mut() {
            if !font.borrow_mut().load_async(&self.gl) {
                ready = false;
            }
        }
        if ready {
            self.ready = true;
        }
        ready
    }
}
