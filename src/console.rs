use crate::font::Font;
use crate::Buffer;
use std::cell::RefCell;
use std::rc::Rc;
use uni_gl::WebGLRenderingContext;

/// This contains the data for a console (including the one displayed on the screen) and methods to draw on it.
pub struct Console {
    pub buffer: Buffer,

    extents: (f32, f32, f32, f32),

    // program: Option<Program>,
    font: Rc<RefCell<Font>>,
    // glyphs: GlyphMap,

    // font_img_width: u32,
    // font_img_height: u32,
    // font_char_width: u32,
    // font_char_height: u32,
    // font_len: u32,

    // ready: bool,
    // font_path: Option<String>,
    // font_texture: uni_gl::WebGLTexture,
    // font_loader: FontLoader,
}

impl Console {
    /// create a new offscreen console that you can blit on another console
    /// width and height are in cells (characters), not pixels.
    pub fn new(width: u32, height: u32, font: Rc<RefCell<Font>>) -> Self {
        // let font = create_texture(gl);
        // // TODO - INDEX!!!!
        // let program = Program::new(app.gl(), 0, DORYEN_VS, DORYEN_FS);

        Self {
            buffer: Buffer::new(width, height),
            // colors: HashMap::new(),
            // color_stack: Vec::new(),
            extents: (0.0, 0.0, 1.0, 1.0),

            // program: Some(program),
            font,
            // glyphs: GlyphMap::new(),

            // font_img_width: 0,
            // font_img_height: 0,
            // font_char_width: 0,
            // font_char_height: 0,
            // font_len: 0,

            // font_path: Some(font_path.to_owned()),
            // ready: false,
            // font_loader: FontLoader::new(),
            // font_texture: create_font_texture(app.gl()),
        }

        // con.setup_font(app);
        // con
    }

    // pub fn to_glyph(&self, ch: char) -> Glyph {
    //     ch as u32
    // }

    pub fn extents(mut self, left: f32, top: f32, right: f32, bottom: f32) -> Self {
        self.set_extents(left, top, right, bottom);
        self
    }

    pub fn set_extents(&mut self, left: f32, top: f32, right: f32, bottom: f32) -> &mut Self {
        println!("console extents = {},{} - {},{}", left, top, right, bottom);

        self.extents = (left, top, right, bottom);
        // if let Some(program) = self.font.borrow_mut().program.as_mut() {
        //     program.set_extents(left, top, right, bottom);
        // }
        self
    }

    pub fn ready(&self) -> bool {
        self.font.borrow().ready()
    }

    // pub fn set_glyphs(&mut self, glyphs: &GlyphMap) -> &mut Self {
    //     self.glyphs = glyphs.clone();
    //     self
    // }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    pub fn get_width(&self) -> u32 {
        self.buffer.get_width()
    }
    pub fn get_height(&self) -> u32 {
        self.buffer.get_height()
    }
    pub fn get_size(&self) -> (u32, u32) {
        (self.get_width(), self.get_height())
    }
    pub fn get_pot_width(&self) -> u32 {
        self.buffer.get_pot_width()
    }
    pub fn get_pot_height(&self) -> u32 {
        self.buffer.get_pot_height()
    }

    /// resizes the console
    pub fn resize(&mut self, width: u32, height: u32) {
        self.buffer.resize(width, height);
    }

    pub fn render(&mut self, gl: &WebGLRenderingContext) {
        let mut font = self.font.borrow_mut();
        font.render(gl, &self.extents, &self.buffer);
    }

    /// returns the cell that the screen pos converts to for this console [0.0-1.0]
    pub fn cell_pos(&self, screen_pct: (f32, f32)) -> Option<(f32, f32)> {
        if screen_pct.0 < self.extents.0 {
            return None;
        }
        if screen_pct.1 < self.extents.1 {
            return None;
        }
        if screen_pct.0 > self.extents.2 {
            return None;
        }
        if screen_pct.1 > self.extents.3 {
            return None;
        }

        let cell_pct = (
            (screen_pct.0 - self.extents.0) / (self.extents.2 - self.extents.0),
            (screen_pct.1 - self.extents.1) / (self.extents.3 - self.extents.1),
        );

        Some((
            (cell_pct.0) * self.buffer.get_width() as f32,
            (cell_pct.1) * self.buffer.get_height() as f32,
        ))
    }
}
