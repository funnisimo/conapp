use super::input::AppInput;
use super::Font;
use crate::simple::Program;
use crate::{FileLoader, Image, RGBA};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uni_gl::{BufferBit, WebGLRenderingContext};

pub(crate) static SUBCELL_BYTES: &[u8] = include_bytes!("../resources/subcell.png");

pub struct AppContext {
    // pub(super) cons: Vec<Console>,
    pub(crate) input: AppInput,
    pub(crate) fps: Fps,
    pub(crate) screen_size: (u32, u32),
    pub(crate) frame_time_ms: f64,
    pub(crate) gl: WebGLRenderingContext,
    pub(crate) fonts: HashMap<String, Rc<RefCell<Font>>>,
    pub(crate) images: HashMap<String, Rc<RefCell<Image>>>,
    pub(crate) ready: bool,
    pub(crate) file_loader: FileLoader,
    pub(crate) simple_program: Program,
}

impl AppContext {
    pub(crate) fn new(
        gl: WebGLRenderingContext,
        screen_size: (u32, u32),
        input: AppInput,
        fps_goal: u32,
    ) -> Self {
        let sub_cell_font = Rc::new(RefCell::new(Font::from_bytes(SUBCELL_BYTES, &gl)));
        let mut fonts = HashMap::new();
        fonts.insert("SUBCELL".to_owned(), sub_cell_font);

        let program = Program::new(&gl);

        AppContext {
            input,
            fps: Fps::new(fps_goal),
            screen_size: screen_size,
            frame_time_ms: 0.0,
            gl,
            fonts,
            images: HashMap::new(),
            ready: false,
            file_loader: FileLoader::new(),
            simple_program: program,
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
            let id = font.borrow().id;
            if !font.borrow().is_loaded() && self.file_loader.check_file_ready(id) {
                let buf = self.file_loader.get_file_content(id);
                font.borrow_mut().load_font_img(&buf, &self.gl);
            } else {
                ready = false;
            }
        }
        for (_, image) in self.images.iter_mut() {
            let id = image.borrow().id;
            if !image.borrow().is_loaded() && self.file_loader.check_file_ready(id) {
                let buf = self.file_loader.get_file_content(id);
                image.borrow_mut().intialize_image(&buf)
            } else {
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

    pub fn input(&self) -> &AppInput {
        &self.input
    }

    pub fn fps(&self) -> u32 {
        self.fps.current()
    }

    pub fn average_fps(&self) -> u32 {
        self.fps.average()
    }

    pub fn frame_time_ms(&self) -> f64 {
        self.frame_time_ms
    }

    pub fn get_screen_size(&self) -> (u32, u32) {
        self.screen_size
    }

    pub fn load_font(&mut self, fontpath: &str) -> Result<Rc<RefCell<Font>>, String> {
        if let Some(font) = self.fonts.get(fontpath) {
            return Ok(font.clone());
        }

        let id = self.file_loader.load_file(fontpath)?;

        let font = Rc::new(RefCell::new(Font::new(id, fontpath, &self.gl)));
        self.fonts.insert(fontpath.to_owned(), font.clone());
        self.ready = false;
        Ok(font)
    }

    pub fn load_image(&mut self, imgpath: &str) -> Result<Rc<RefCell<Image>>, String> {
        if let Some(image) = self.images.get(imgpath) {
            return Ok(image.clone());
        }

        let id = self.file_loader.load_file(imgpath)?;

        let image = Rc::new(RefCell::new(Image::new_async(id)));
        self.images.insert(imgpath.to_owned(), image.clone());
        self.ready = false;
        Ok(image)
    }
}

pub struct Fps {
    counter: u32,
    start: f64,
    last: f64,
    total_frames: u64,
    fps: u32,
    average: u32,
    goal: u32,
}

impl Fps {
    pub fn new(goal: u32) -> Fps {
        let now = crate::app::now();
        Fps {
            counter: 0,
            total_frames: 0,
            start: now,
            last: now,
            fps: 0,
            average: 0,
            goal,
        }
    }

    pub fn goal(&self) -> u32 {
        self.goal
    }

    pub fn current(&self) -> u32 {
        self.fps
    }

    pub fn step(&mut self) {
        self.counter += 1;
        self.total_frames += 1;
        let curr = crate::app::now();
        if curr - self.last > 1.0 {
            self.last = curr;
            self.fps = self.counter;
            self.counter = 0;
            self.average = (self.total_frames as f64 / (self.last - self.start)) as u32;
        }
    }
    pub fn average(&self) -> u32 {
        self.average
    }
}
