use super::DoryenUniforms;
use super::*;
use super::{Program, DORYEN_FS, DORYEN_VS};
use crate::AppContext;
use crate::Buffer;
use image::{ImageBuffer, Rgba};
use uni_gl::WebGLRenderingContext;

pub struct Font {
    img_width: u32,
    img_height: u32,
    char_width: u32,
    char_height: u32,
    len: u32,

    path: String,
    loaded: bool,
    // path: Option<String>,
    loader: FontLoader,

    pub program: Option<Program>,
}

impl Font {
    pub fn new(path: &str, app: &mut dyn AppContext) -> Self {
        let mut loader = FontLoader::new();
        crate::App::print(format!("Loading font - {}", path));
        loader.load_font(path);

        // TODO - INDEX!!!!
        let program = Program::new(app.gl(), 0, DORYEN_VS, DORYEN_FS);

        Font {
            img_width: 0,
            img_height: 0,
            char_width: 0,
            char_height: 0,
            len: 0,

            path: path.to_owned(),
            loaded: false,
            loader,

            program: Some(program),
        }
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

    pub fn loaded(&self) -> bool {
        self.loaded
    }

    fn take_img(&self) -> &ImageBuffer<Rgba<u8>, Vec<u8>> {
        self.loader.img.as_ref().unwrap()
    }

    pub fn load_async(&mut self, gl: &WebGLRenderingContext) -> bool {
        if self.loaded {
            return true;
        }

        if !self.loader.load_font_async() {
            crate::App::print(format!("-loading font: {}", self.path));
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

        crate::app::App::print(format!(
            "Font loaded: {} -> font size: {:?} char size: {:?} len: {:?}",
            self.path.as_str(),
            (self.img_width, self.img_height),
            (self.char_width, self.char_height),
            self.len()
        ));
    }

    // TODO - Move to Program
    fn setup_font(&mut self, gl: &WebGLRenderingContext) {
        if let Some(mut program) = self.program.take() {
            gl.use_program(&program.program);

            // TODO - INDEX!!!
            gl.active_texture(0);
            // gl.active_texture(self.index);

            gl.bind_texture(&program.font);
            {
                let img = self.take_img();
                gl.tex_image2d(
                    uni_gl::TextureBindPoint::Texture2d, // target
                    0,                                   // level
                    img.width() as u16,                  // width
                    img.height() as u16,                 // height
                    uni_gl::PixelFormat::Rgba,           // format
                    uni_gl::PixelType::UnsignedByte,     // type
                    &*img,                               // data
                );
            }

            // program.bind(
            //     gl,
            //     &self,
            //     self.img_width(),
            //     self.img_height(),
            //     self.char_width(),
            //     self.char_height(),
            // );

            if let Some(&Some(ref location)) = program
                .uniform_locations
                .get(&DoryenUniforms::FontCharsPerLine)
            {
                gl.uniform_1f(
                    location,
                    (self.img_width() as f32) / (self.char_width() as f32),
                );
            }
            if let Some(&Some(ref location)) =
                program.uniform_locations.get(&DoryenUniforms::FontCoef)
            {
                gl.uniform_2f(
                    location,
                    (
                        (self.char_width() as f32) / (self.img_width() as f32),
                        (self.char_height() as f32) / (self.img_height() as f32),
                    ),
                );
            }

            program.set_font_texture(gl);
            self.program = Some(program);
        }
    }

    pub fn render(
        &mut self,
        gl: &WebGLRenderingContext,
        extents: &(f32, f32, f32, f32),
        buffer: &Buffer,
    ) {
        if let Some(mut program) = self.program.take() {
            program.set_extents(gl, extents.0, extents.1, extents.2, extents.3);
            program.render_primitive(gl, buffer);
            self.program = Some(program);
        }
    }
}
