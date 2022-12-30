use super::{create_font_texture, Program, DORYEN_FS, DORYEN_VS};
use crate::Buffer;
use uni_gl::{WebGLRenderingContext, WebGLTexture};

pub struct Font {
    pub(crate) id: usize,
    img_width: u32,
    img_height: u32,
    char_width: u32,
    char_height: u32,
    len: u32,

    path: String,
    loaded: bool,
    // path: Option<String>,
    // loader: FontLoader,
    // pub(crate) img: Option<image::RgbaImage>,
    program: Option<Program>,

    pub(crate) texture: WebGLTexture,
}

impl Font {
    pub(crate) fn new(id: usize, path: &str, gl: &WebGLRenderingContext) -> Self {
        // let mut loader = FontLoader::new();
        // crate::console(&format!("Loading font - {}", path));
        // loader.load_font(path);

        let program = Program::new(gl, DORYEN_VS, DORYEN_FS);

        let (char_width, char_height) = parse_char_size(path);

        Font {
            id,
            img_width: 0,
            img_height: 0,
            char_width,
            char_height,
            len: 0,

            path: path.to_owned(),
            loaded: false,
            // loader,
            // img: None,
            program: Some(program),
            texture: create_font_texture(gl),
        }
    }

    pub(crate) fn from_bytes(bytes: &[u8], gl: &WebGLRenderingContext) -> Self {
        // let mut loader = FontLoader::new();
        // crate::console("Loading font from bytes");
        // loader.load_bytes(bytes);

        let program = Program::new(gl, DORYEN_VS, DORYEN_FS);

        let mut font = Font {
            id: 0,
            // index,
            img_width: 0,
            img_height: 0,
            char_width: 4,
            char_height: 4,
            len: 0,

            path: "bytes".to_owned(),
            loaded: false,
            // loader,
            // img: None,
            program: Some(program),
            texture: create_font_texture(gl),
        };

        font.load_font_img(bytes, gl);
        // font.setup_font(gl);
        font.loaded = true;

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

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    // pub fn img(&self) -> Option<&image::RgbaImage> {
    //     self.img.as_ref()
    // }

    // pub(crate) fn load_async(&mut self, gl: &WebGLRenderingContext) -> bool {
    //     if self.loaded {
    //         return true;
    //     }

    //     if !self.loader.load_font_async() {
    //         crate::console(&format!("- still loading font: {}", self.path));
    //         return false;
    //     }

    //     self.load_font_info();
    //     self.setup_font(gl);
    //     self.loaded = true;
    //     true
    // }

    pub(crate) fn load_font_img(&mut self, buf: &[u8], gl: &WebGLRenderingContext) {
        let mut img = image::load_from_memory(&buf).unwrap().to_rgba8();
        process_image(&mut img);

        self.img_width = img.width() as u32;
        self.img_height = img.height() as u32;
        self.len = (self.img_width / self.char_width) * (self.img_height / self.char_height);

        // self.img = Some(img);

        // if let Some(mut program) = self.program.take() {
        //     program.set_font_texture(gl, &img);
        //     self.program = Some(program);
        //     println!("Loaded font program - {}", &self.path);
        // }

        gl.bind_texture(&self.texture);

        gl.tex_image2d(
            uni_gl::TextureBindPoint::Texture2d, // target
            0,                                   // level
            img.width() as u16,                  // width
            img.height() as u16,                 // height
            uni_gl::PixelFormat::Rgba,           // format
            uni_gl::PixelType::UnsignedByte,     // type
            &*img,                               // data
        );

        crate::console(&format!(
            "Font loaded: {} -> font size: {:?} char size: {:?} len: {:?}",
            self.path.as_str(),
            (self.img_width, self.img_height),
            (self.char_width, self.char_height),
            self.len()
        ));

        self.loaded = true;
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
            program.use_font(gl, &self);
            program.set_extents(gl, extents.0, extents.1, extents.2, extents.3);
            program.render_buffer(gl, buffer);
            self.program = Some(program);
        }
    }
}

fn parse_char_size(filepath: &str) -> (u32, u32) {
    let mut char_width = 0;
    let mut char_height = 0;

    let start = match filepath.rfind('_') {
        None => {
            panic!("Failed to load font.  Font file name must end with cell size information ('_8x8.' in 'name_8x8.png') - {}", filepath);
        }
        Some(idx) => idx,
    };
    let end = match filepath.rfind('.') {
        None => {
            panic!("Failed to load font.  Font file name must end with cell size information ('_8x8.' in 'name_8x8.png') - {}", filepath);
        }
        Some(idx) => idx,
    };
    if start > 0 && end > 0 {
        let subpath = &filepath[start + 1..end];
        let charsize: Vec<&str> = subpath.split('x').collect();
        char_width = match charsize[0].parse::<u32>() {
            Err(_) => {
                panic!("Failed to load font.  Font file name must end with cell size information ('_8x8.' in 'name_8x8.png') - {}", filepath);
            }
            Ok(val) => val,
        };
        char_height = match charsize[1].parse::<u32>() {
            Err(_) => {
                panic!("Failed to load font.  Font file name must end with cell size information ('_8x8.' in 'name_8x8.png') - {}", filepath);
            }
            Ok(val) => val,
        };
    }
    (char_width, char_height)
}

fn process_image(img: &mut image::RgbaImage) {
    let pixel = img.get_pixel(0, 0);
    let alpha = pixel[3];
    if alpha == 255 {
        let transparent_color = (pixel[0], pixel[1], pixel[2]);
        let greyscale = transparent_color == (0, 0, 0);
        crate::console(&format!(
            "{}transparent color: {:?}",
            if greyscale { "greyscale " } else { "" },
            transparent_color
        ));
        let (width, height) = img.dimensions();
        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel_mut(x, y);
                if (pixel[0], pixel[1], pixel[2]) == transparent_color {
                    pixel[3] = 0;
                    pixel[0] = 0;
                    pixel[1] = 0;
                    pixel[2] = 0;
                } else if greyscale && pixel[0] == pixel[1] && pixel[1] == pixel[2] {
                    let alpha = pixel[0];
                    pixel[0] = 255;
                    pixel[1] = 255;
                    pixel[2] = 255;
                    pixel[3] = alpha;
                }
            }
        }
    }
}
