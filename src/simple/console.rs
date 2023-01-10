use super::Buffer;
use crate::font::Font;
use crate::{console, AppContext};
use std::rc::Rc;

/// This contains the data for a console (including the one displayed on the screen) and methods to draw on it.
pub struct Console {
    buffer: Buffer,
    extents: (f32, f32, f32, f32),
    fontpath: String,
    font: Option<Rc<Font>>,
    zpos: i8,
}

impl Console {
    /// create a new offscreen console that you can draw to the screen with a font
    /// width and height are in cells (characters), not pixels.
    pub fn new(width: u32, height: u32, font: &str) -> Self {
        Self {
            buffer: Buffer::new(width, height),
            extents: (0.0, 0.0, 1.0, 1.0),
            fontpath: font.to_owned(),
            font: None,
            zpos: 0,
        }
    }

    pub fn with_extents(mut self, left: f32, top: f32, right: f32, bottom: f32) -> Self {
        self.set_extents(left, top, right, bottom);
        self
    }

    pub fn with_zpos(mut self, zpos: i8) -> Self {
        self.zpos = zpos;
        self
    }

    pub fn set_extents(&mut self, left: f32, top: f32, right: f32, bottom: f32) -> &mut Self {
        println!("console extents = {},{} - {},{}", left, top, right, bottom);

        self.extents = (left, top, right, bottom);
        self
    }

    pub fn set_zpos(&mut self, zpos: i8) -> &mut Self {
        self.zpos = zpos;
        self
    }

    pub fn ready(&self) -> bool {
        self.font.is_some()
    }

    pub fn set_font(&mut self, font: Rc<Font>) {
        self.font = Some(font);
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    pub fn width(&self) -> u32 {
        self.buffer.width()
    }
    pub fn height(&self) -> u32 {
        self.buffer.height()
    }
    pub fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }
    pub fn pot_width(&self) -> u32 {
        self.buffer.pot_size().0
    }
    pub fn pot_height(&self) -> u32 {
        self.buffer.pot_size().1
    }

    pub fn font_char_size(&self) -> (u32, u32) {
        match self.font {
            None => (0, 0),
            Some(ref f) => f.char_size(),
        }
    }

    /// resizes the console
    pub fn resize(&mut self, width: u32, height: u32) {
        self.buffer.resize(width, height);
    }

    pub fn render(&mut self, app: &mut AppContext) {
        match self.font {
            None => {
                self.font = app.get_font(self.fontpath.as_ref());
                if self.font.is_some() {
                    console(format!("Got font - {}", self.fontpath));
                }
            }
            Some(ref font) => {
                let gl = &app.gl;
                let program = &mut app.simple_program;
                program.use_font(gl, &font);
                program.set_extents(gl, &self.extents, self.zpos);
                program.render_buffer(gl, &self.buffer);

                // font.render(gl, &self.extents, &self.buffer);
            }
        }
    }

    /// returns the cell that the screen pos converts to for this console [0.0-1.0]
    pub fn mouse_pos(&self, screen_pct: (f32, f32)) -> Option<(f32, f32)> {
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
            (cell_pct.0) * self.buffer.width() as f32,
            (cell_pct.1) * self.buffer.height() as f32,
        ))
    }
}

pub fn subcell_console(width: u32, height: u32) -> Console {
    Console::new(width, height, "SUBCELL")
}

pub fn default_console(width: u32, height: u32) -> Console {
    Console::new(width, height, "DEFAULT")
}

pub fn calc_window_pct(
    screen_size_px: (u32, u32),
    char_size_px: (u32, u32),
    window_size_cells: (u32, u32),
) -> (f32, f32) {
    let window_size_px = (
        window_size_cells.0 * char_size_px.0,
        window_size_cells.1 * char_size_px.1,
    );
    (
        (window_size_px.0 as f32 / screen_size_px.0 as f32).min(1.0),
        (window_size_px.1 as f32 / screen_size_px.1 as f32).min(1.0),
    )
}

pub fn calc_center_offset(window_size_pct: (f32, f32)) -> (f32, f32) {
    (
        (1.0 - window_size_pct.0).max(0.0) / 2.0,
        (1.0 - window_size_pct.1).max(0.0) / 2.0,
    )
}

pub fn calc_center_extents(window_size_pct: (f32, f32)) -> (f32, f32, f32, f32) {
    let offset = calc_center_offset(window_size_pct);
    (
        offset.0,
        offset.1,
        offset.0 + window_size_pct.0,
        offset.1 + window_size_pct.1,
    )
}
