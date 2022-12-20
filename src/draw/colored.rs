use super::TextAlign;
use crate::codepage437;
use crate::Buffer;
use crate::Glyph;
use crate::{parse_color, RGBA};
use std::cmp::max;

pub fn colored<'a>(buffer: &'a mut Buffer) -> ColoredPrinter {
    ColoredPrinter::new(buffer)
}

pub struct ColoredPrinter<'a> {
    buffer: &'a mut Buffer,
    width: Option<i32>,
    align: TextAlign,
    fg: Option<RGBA>,
    bg: Option<RGBA>,
    to_glyph: &'a dyn Fn(char) -> Glyph,
    to_rgba: &'a dyn Fn(&str) -> Option<RGBA>,
}

impl<'a> ColoredPrinter<'a> {
    pub fn new(buffer: &'a mut Buffer) -> Self {
        ColoredPrinter {
            buffer,
            width: None,
            align: TextAlign::Left,
            fg: None,
            bg: None,
            to_glyph: &codepage437::to_glyph,
            to_rgba: &parse_color,
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

    pub fn to_rgba(mut self, to_rgba: &'a dyn Fn(&str) -> Option<RGBA>) -> Self {
        self.to_rgba = to_rgba;
        self
    }

    pub fn print(&mut self, x: i32, y: i32, text: &str) -> i32 {
        let chars: Vec<(Option<RGBA>, char)> = TextIterator::new(self.to_rgba, text).collect();
        self.print_line(x, y, chars)
    }

    fn print_line(&mut self, x: i32, y: i32, chars: Vec<(Option<RGBA>, char)>) -> i32 {
        let mut width = self.width.unwrap_or(chars.len() as i32);
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
        let mut iter = chars.iter().skip(start as usize);
        for _ in 0..width {
            let (color, glyph) = match iter.next() {
                None => (None, None),
                Some((color, ch)) => (*color, Some((self.to_glyph)(*ch))),
            };
            self.buffer
                .draw_opt(ix, y, glyph, color.or(self.fg), self.bg);
            ix += 1;
        }
        width
    }

    pub fn print_lines(&mut self, x: i32, y: i32, text: &str) -> (i32, i32) {
        let chars: Vec<(Option<RGBA>, char)> = TextIterator::new(self.to_rgba, text).collect();

        let lines = chars
            .into_iter()
            .fold(vec![Vec::new()], |mut acc, (color, ch)| {
                if ch == '\n' {
                    acc.push(Vec::new());
                } else {
                    acc.last_mut().unwrap().push((color, ch));
                }
                acc
            });

        let mut width = 0;
        let mut height = 0;
        for line in lines {
            let w = self.print_line(x, y + height, line);
            width = max(width, w);
            height += 1;
        }
        (width, height)
    }
}

#[derive(Debug, Clone)]
struct ColoredTextPart<'a> {
    txt: &'a str,
    color: Option<RGBA>,
}

impl<'a> ColoredTextPart<'a> {
    fn new(color: Option<RGBA>, txt: &'a str) -> Self {
        ColoredTextPart { txt, color }
    }
}

fn parse_colored_text<'a>(
    to_rgba: &'a dyn Fn(&str) -> Option<RGBA>,
    txt: &'a str,
) -> Vec<ColoredTextPart<'a>> {
    let mut colors: Vec<Option<RGBA>> = Vec::new();
    let mut out: Vec<ColoredTextPart<'a>> = Vec::new();
    let default_color: Option<RGBA> = None;

    for (i, major_part) in txt.split("#[").enumerate() {
        if major_part.len() == 0 {
            continue;
        } // skip empty parts
        if i == 0 {
            out.push(ColoredTextPart::new(default_color, major_part));
        } else if major_part.starts_with("[") {
            let c = colors.iter().last().unwrap_or(&default_color);
            out.push(ColoredTextPart::new(c.clone(), "#["));
            out.push(ColoredTextPart::new(c.clone(), &major_part[1..]));
        } else {
            let inner_parts: Vec<&'a str> = major_part.splitn(2, "]").collect();
            if inner_parts.len() != 2 {
                panic!("Parsing error! - {}", txt);
            }
            if inner_parts[0].len() == 0 {
                colors.pop();
            } else {
                let c = to_rgba(inner_parts[0]);
                colors.push(c);
            }
            let c = colors.iter().last().unwrap_or(&default_color);
            out.push(ColoredTextPart::new(c.clone(), inner_parts[1]));
        }
    }

    // println!("- {:?}", out);
    // println!("--");
    out
}

pub struct TextIterator<'a> {
    data: Vec<ColoredTextPart<'a>>,
}

impl<'a> TextIterator<'a> {
    pub fn new(to_rgba: &'a dyn Fn(&str) -> Option<RGBA>, txt: &'a str) -> Self {
        let data = parse_colored_text(to_rgba, txt);
        TextIterator { data }
    }
}

impl<'a> Iterator for TextIterator<'a> {
    type Item = (Option<RGBA>, char);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.data.len() == 0 {
                return None;
            }

            let mut pop = false;
            if let Some(part) = self.data.first_mut() {
                if part.txt.len() == 0 {
                    pop = true;
                } else {
                    let ch = part.txt.chars().nth(0).unwrap();
                    part.txt = &part.txt[1..];
                    return Some((part.color, ch));
                }
            }
            if pop {
                self.data.remove(0);
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const WHITE: RGBA = RGBA::rgb(255, 255, 255);
    const RED: RGBA = RGBA::rgb(255, 0, 0);
    const _GREEN: RGBA = RGBA::rgb(0, 255, 0);
    const BLUE: RGBA = RGBA::rgb(0, 0, 255);
    const _BLACK: RGBA = RGBA::rgb(0, 0, 0);

    #[test]
    fn no_color() {
        let mut iter = TextIterator::new(&|_| Some(WHITE), "Text");

        assert_eq!(iter.next().unwrap(), (None, 'T'));
        assert_eq!(iter.next().unwrap(), (None, 'e'));
        assert_eq!(iter.next().unwrap(), (None, 'x'));
        assert_eq!(iter.next().unwrap(), (None, 't'));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn start_color() {
        fn to_rgba(_: &str) -> Option<RGBA> {
            Some(BLUE)
        }

        let mut iter = TextIterator::new(&to_rgba, "#[blue]Text");

        assert_eq!(iter.next().unwrap(), (Some(BLUE), 'T'));
        assert_eq!(iter.next().unwrap(), (Some(BLUE), 'e'));
        assert_eq!(iter.next().unwrap(), (Some(BLUE), 'x'));
        assert_eq!(iter.next().unwrap(), (Some(BLUE), 't'));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn mid_color() {
        fn to_rgba(t: &str) -> Option<RGBA> {
            match t {
                "blue" => Some(BLUE),
                "white" => Some(WHITE),
                _ => None,
            }
        }

        let mut iter = TextIterator::new(&to_rgba, "a #[blue]b#[] c");

        assert_eq!(iter.next().unwrap(), (None, 'a'));
        assert_eq!(iter.next().unwrap(), (None, ' '));
        assert_eq!(iter.next().unwrap(), (Some(BLUE), 'b'));
        assert_eq!(iter.next().unwrap(), (None, ' '));
        assert_eq!(iter.next().unwrap(), (None, 'c'));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn escape_color() {
        let mut iter = TextIterator::new(&|_| Some(RED), "a #[[blue]b#[[] c");

        assert_eq!(iter.next().unwrap(), (None, 'a'));
        assert_eq!(iter.next().unwrap(), (None, ' '));
        assert_eq!(iter.next().unwrap(), (None, '#'));
        assert_eq!(iter.next().unwrap(), (None, '['));
        assert_eq!(iter.next().unwrap(), (None, 'b'));
        assert_eq!(iter.next().unwrap(), (None, 'l'));
        assert_eq!(iter.next().unwrap(), (None, 'u'));
        assert_eq!(iter.next().unwrap(), (None, 'e'));
        assert_eq!(iter.next().unwrap(), (None, ']'));
        assert_eq!(iter.next().unwrap(), (None, 'b'));
        assert_eq!(iter.next().unwrap(), (None, '#'));
        assert_eq!(iter.next().unwrap(), (None, '['));
        assert_eq!(iter.next().unwrap(), (None, ']'));
        assert_eq!(iter.next().unwrap(), (None, ' '));
        assert_eq!(iter.next().unwrap(), (None, 'c'));
        assert_eq!(iter.next(), None);
    }
}
