use super::input::{AppInput, InputApi};
use super::Font;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uni_gl::WebGLRenderingContext;

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

    fn ready(&mut self) -> bool;
    // fn load_font(&mut self, fontpath: &str);
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

    // fn clear_all(&mut self) -> () {
    //     for cons in self.cons.iter_mut() {
    //         cons.buffer_mut()
    //             .clear(Some(RGBA::new()), Some(RGBA::new()), Some(0));
    //     }
    // }

    fn ready(&mut self) -> bool {
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

    fn get_font(&self, fontpath: &str) -> Option<Rc<RefCell<Font>>> {
        match self.fonts.get(fontpath) {
            None => None,
            Some(font) => Some(font.clone()),
        }
    }
}

impl ContextImpl {
    // pub(super) fn render(&mut self, gl: &uni_gl::WebGLRenderingContext) {
    //     // for con in self.cons.iter_mut() {
    //     //     con.render(gl);
    //     // }
    // }

    pub fn resize(&mut self, screen_width: u32, screen_height: u32) {
        self.screen_size = (screen_width, screen_height);
        // for con in self.cons.iter_mut() {
        //     con.screen_resize(gl, screen_width, screen_height);
        // }
    }

    pub fn load_font(&mut self, fontpath: &str) {
        if self.fonts.contains_key(fontpath) {
            return;
        }

        let font = Rc::new(RefCell::new(Font::new(fontpath, self)));
        self.fonts.insert(fontpath.to_owned(), font);
        self.ready = false;
    }
}
