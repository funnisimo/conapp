use super::*;
use super::{Program, DORYEN_FS, DORYEN_VS};
use crate::Buffer;
use image::{ImageBuffer, Rgba};
use std::cell::RefCell;
use std::rc::Rc;
use uni_gl::WebGLRenderingContext;

pub struct Font {
    // index: u32,
    img_width: u32,
    img_height: u32,
    char_width: u32,
    char_height: u32,
    len: u32,

    path: String,
    loaded: bool,
    // path: Option<String>,
    loader: FontLoader,

    program: Option<Program>,
}

impl Font {
    pub(crate) fn new(path: &str, gl: &WebGLRenderingContext) -> Rc<RefCell<Self>> {
        let mut loader = FontLoader::new();
        crate::console(&format!("Loading font - {}", path));
        loader.load_font(path);

        let program = Program::new(gl, DORYEN_VS, DORYEN_FS);

        Rc::new(RefCell::new(Font {
            // index,
            img_width: 0,
            img_height: 0,
            char_width: 0,
            char_height: 0,
            len: 0,

            path: path.to_owned(),
            loaded: false,
            loader,

            program: Some(program),
        }))
    }

    pub(crate) fn from_bytes(bytes: &[u8], gl: &WebGLRenderingContext) -> Rc<RefCell<Self>> {
        let mut loader = FontLoader::new();
        crate::console("Loading font from bytes");
        loader.load_bytes(bytes, 4, 4);

        let program = Program::new(gl, DORYEN_VS, DORYEN_FS);

        let font = Rc::new(RefCell::new(Font {
            // index,
            img_width: 0,
            img_height: 0,
            char_width: 0,
            char_height: 0,
            len: 0,

            path: "bytes".to_owned(),
            loaded: false,
            loader,

            program: Some(program),
        }));

        font.borrow_mut().load_font_info();
        font.borrow_mut().setup_font(gl);
        font.borrow_mut().loaded = true;

        font
    }

    pub fn img_width(&self) -> u32 {
        self.img_width
    }
    pub fn img_height(&self) -> u32 {
        self.img_height
    }
    pub fn char_width(&self) -> u32 {
        self.char_width
    }
    pub fn char_height(&self) -> u32 {
        self.char_height
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn ready(&self) -> bool {
        self.loaded
    }

    fn take_img(&self) -> &ImageBuffer<Rgba<u8>, Vec<u8>> {
        self.loader.img.as_ref().unwrap()
    }

    pub(crate) fn load_async(&mut self, gl: &WebGLRenderingContext) -> bool {
        if self.loaded {
            return true;
        }

        if !self.loader.load_font_async() {
            crate::console(&format!("- still loading font: {}", self.path));
            return false;
        }

        self.load_font_info();
        self.setup_font(gl);
        self.loaded = true;
        true
    }

    fn load_font_info(&mut self) {
        let img = self.loader.img.as_ref().unwrap();
        if self.loader.char_width != 0 {
            self.char_width = self.loader.char_width;
            self.char_height = self.loader.char_height;
        } else {
            self.char_width = img.width() as u32 / 16;
            self.char_height = img.height() as u32 / 16;
        }
        self.img_width = img.width() as u32;
        self.img_height = img.height() as u32;

        self.len = (self.img_width / self.char_width) * (self.img_height / self.char_height);

        crate::console(&format!(
            "Font loaded: {} -> font size: {:?} char size: {:?} len: {:?}",
            self.path.as_str(),
            (self.img_width, self.img_height),
            (self.char_width, self.char_height),
            self.len()
        ));
    }

    fn setup_font(&mut self, gl: &WebGLRenderingContext) {
        if let Some(mut program) = self.program.take() {
            program.set_font_texture(
                gl,
                self.take_img(),
                self.img_width(),
                self.img_height(),
                self.char_width(),
                self.char_height(),
            );
            self.program = Some(program);
        }
    }

    pub fn render(
        &mut self,
        gl: &WebGLRenderingContext,
        extents: &(f32, f32, f32, f32),
        buffer: &Buffer,
    ) {
        if !self.loaded {
            return;
        }
        if let Some(mut program) = self.program.take() {
            program.set_extents(gl, extents.0, extents.1, extents.2, extents.3);
            program.render_primitive(gl, buffer);
            self.program = Some(program);
        }
    }
}
