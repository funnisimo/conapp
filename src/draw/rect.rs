use crate::{Buffer, Glyph, RGBA};

pub fn rect<'a>(buffer: &'a mut Buffer) -> RectPrinter {
    RectPrinter::new(buffer)
}

pub struct RectPrinter<'a> {
    buffer: &'a mut Buffer,
    fg: Option<RGBA>,
    bg: Option<RGBA>,
    glyph: Option<Glyph>,
}

impl<'a> RectPrinter<'a> {
    pub fn new(buffer: &'a mut Buffer) -> Self {
        RectPrinter {
            buffer,
            fg: None,
            bg: None,
            glyph: None,
        }
    }

    pub fn fg(mut self, fg: RGBA) -> Self {
        self.fg = Some(fg);
        self
    }

    pub fn bg(mut self, bg: RGBA) -> Self {
        self.bg = Some(bg);
        self
    }

    pub fn glyph(mut self, glyph: Glyph) -> Self {
        self.glyph = Some(glyph);
        self
    }

    pub fn draw(&mut self, x: i32, y: i32, width: u32, height: u32) {
        self.buffer
            .area(x, y, width, height, self.glyph, self.fg, self.bg);
    }
}
