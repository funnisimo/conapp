use crate::{codepage437, Buffer, Glyph, RGBA};
use std::cmp::max;

#[derive(Copy, Clone)]
pub enum TextAlign {
    Left,
    Right,
    Center,
}

pub fn plain<'a>(buffer: &'a mut Buffer) -> PlainPrinter {
    PlainPrinter::new(buffer)
}

pub struct PlainPrinter<'a> {
    buffer: &'a mut Buffer,
    width: Option<i32>,
    align: TextAlign,
    fg: Option<RGBA>,
    bg: Option<RGBA>,
    to_glyph: &'a dyn Fn(char) -> Glyph,
}

impl<'a> PlainPrinter<'a> {
    pub fn new(buffer: &'a mut Buffer) -> Self {
        PlainPrinter {
            buffer,
            width: None,
            align: TextAlign::Left,
            fg: None,
            bg: None,
            to_glyph: &codepage437::to_glyph,
        }
    }

    pub fn width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    pub fn fg(mut self, fg: RGBA) -> Self {
        self.fg = Some(fg);
        self
    }

    pub fn bg(mut self, bg: RGBA) -> Self {
        self.bg = Some(bg);
        self
    }

    pub fn to_glyph(mut self, to_glyph: &'a dyn Fn(char) -> Glyph) -> Self {
        self.to_glyph = to_glyph;
        self
    }

    pub fn print(&mut self, x: i32, y: i32, text: &str) -> i32 {
        let mut width = self.width.unwrap_or(text.chars().count() as i32);
        let mut start = 0;
        let mut ix = match self.align {
            TextAlign::Left => x,
            TextAlign::Right => x - width + 1,
            TextAlign::Center => x - width / 2,
        };
        if ix < 0 {
            width += ix;
            start -= ix;
            ix = 0;
        }
        if ix + width > self.buffer.get_width() as i32 {
            width = self.buffer.get_width() as i32 - ix;
        }
        let mut chars = text.chars().skip(start as usize);
        for _ in 0..width {
            let glyph = match chars.next() {
                None => None,
                Some(ch) => Some((self.to_glyph)(ch)),
            };
            self.buffer.draw_opt(ix, y, glyph, self.fg, self.bg);
            ix += 1;
        }
        width
    }

    pub fn print_lines(&mut self, x: i32, y: i32, text: &str) -> (i32, i32) {
        let mut width = 0;
        let mut height = 0;
        for line in text.split('\n') {
            let w = self.print(x, y + height, line);
            width = max(width, w);
            height += 1;
        }
        (width, height)
    }
}
