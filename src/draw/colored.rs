use super::TextAlign;
use crate::codepage437;
use crate::Buffer;
use crate::Glyph;
use crate::{to_rgba, RGBA};
use std::cmp::{max, min};

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
            fg: Some(RGBA::rgb(255, 255, 255)),
            bg: None,
            to_glyph: &codepage437::to_glyph,
            to_rgba: &to_rgba,
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

    fn print_char(&mut self, x: i32, y: i32, ch: Option<char>, fg: Option<RGBA>) {
        let glyph = match ch {
            None => Some(0),
            Some(ch) => Some((self.to_glyph)(ch)),
        };
        self.buffer.draw_opt(x, y, glyph, fg.or(self.fg), self.bg);
    }

    pub fn print(&mut self, x: i32, y: i32, text: &str) -> i32 {
        let chars: Vec<(Option<RGBA>, char)> = TextIterator::new(self.to_rgba, text).collect();
        self.print_line(x, y, &chars)
    }

    fn print_line(&mut self, x: i32, y: i32, chars: &[(Option<RGBA>, char)]) -> i32 {
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
            let (fg, ch) = match iter.next() {
                None => (None, None),
                Some(x) => (x.0, Some(x.1)),
            };
            self.print_char(ix, y, ch, fg);
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
            let w = self.print_line(x, y + height, &line);
            width = max(width, w);
            height += 1;
        }
        (width, height)
    }

    fn print_word(&mut self, x: i32, y: i32, chars: &[(Option<RGBA>, char)]) -> i32 {
        let width = chars.len() as i32;
        let mut iter = chars.iter();
        let mut ix = x;
        for _ in 0..width {
            let (fg, ch) = match iter.next() {
                None => (None, None),
                Some(x) => (x.0, Some(x.1)),
            };
            self.print_char(ix, y, ch, fg);
            ix += 1;
        }
        width
    }

    pub fn wrap(&mut self, x: i32, y: i32, text: &str) -> (i32, i32) {
        let width = self.width.unwrap_or(self.buffer.get_width() as i32 - x);

        let chars: Vec<(Option<RGBA>, char)> = TextIterator::new(self.to_rgba, text).collect();

        let lines_of_words = make_lines_of_words(chars);

        // println!("==========================");
        // println!("WRAP = {}", text);

        let mut widest = 0;
        let mut cx = x;
        let mut cy = y;
        let mut line_left = width;
        let ex = x + width;

        for (i, line) in lines_of_words.iter().enumerate() {
            if i > 0 {
                if self.width.is_some() && self.bg.is_some() {
                    for fx in cx..ex {
                        self.print_char(fx, cy, None, None);
                    }
                }
                widest = max(widest, cx - x);
                cx = x;
                cy += 1;
                line_left = width;
            }

            for (i, word) in line.iter().enumerate() {
                // println!(
                //     "word={:?}, len={}, cx={}, line_left={}",
                //     word,
                //     word.len(),
                //     cx,
                //     line_left
                // );
                let first_fg = word.first().unwrap_or(&(None, ' ')).0;

                if i > 0 && line_left > word.len() as i32 {
                    self.print_char(cx, cy, Some(' '), first_fg);
                    line_left -= 1;
                    cx += 1;
                    // println!("- add space, cx={}, ll={}", cx, line_left);
                }

                if word.len() == 0 {
                    if line_left > 0 {
                        self.print_char(cx, cy, Some(' '), first_fg);
                        line_left -= 1;
                        cx += 1;
                        // println!("- add space, cx={}, ll={}", cx, line_left);
                    }
                } else if (word.len() as i32) <= line_left {
                    let word_len = self.print_word(cx, cy, word);
                    cx += word_len;
                    line_left -= word_len;
                    // println!("- add word, cx={}, ll={}", cx, line_left);
                } else if (word.len() as i32) > width {
                    // We are longer than a single line
                    // Do we fit on this line and the next
                    // println!("- long word");

                    if line_left < 4 {
                        if self.width.is_some() && self.bg.is_some() {
                            for fx in cx..ex {
                                self.print_char(fx, cy, None, None);
                            }
                        }
                        widest = max(widest, cx - x);
                        cx = x;
                        cy += 1;
                        line_left = width;
                        // println!("- push to next line");
                    } else if cx > x {
                        self.print_char(cx, cy, Some(' '), first_fg);
                        line_left -= 1;
                        cx += 1;
                        // println!("- space");
                    }

                    for (fg, ch) in word {
                        if line_left == 1 {
                            self.print_char(cx, cy, Some('-'), *fg);
                            cx += 1;

                            if self.width.is_some() && self.bg.is_some() {
                                for fx in cx..ex {
                                    self.print_char(fx, cy, None, None);
                                }
                            }

                            widest = max(widest, cx - x);
                            cx = x;
                            line_left = width;
                            cy += 1;
                            // println!("- hyphen + next line");
                        }

                        self.print_char(cx, cy, Some(*ch), *fg);
                        line_left -= 1;
                        cx += 1;
                        // println!("- add letter, ch={}, cx={}, ll={}", ch, cx, line_left);
                    }
                } else if word.len() > 6 && line_left - 2 >= word.len() as i32 / 2 {
                    let pivot = min(line_left - 2, word.len() as i32 / 2);

                    let left = &word[..pivot as usize];
                    let right = &word[pivot as usize..];

                    if cx > x {
                        self.print_char(cx, cy, Some(' '), first_fg);
                        // line_left -= 1;
                        cx += 1;
                        // println!("- space");
                    }

                    let len = self.print_word(cx, cy, left);
                    cx += len;
                    // line_left -= len;
                    // println!("- add half: word={:?}, cx={}, ll={}", left, cx, line_left);
                    self.print_char(cx, cy, Some('-'), first_fg);
                    cx += 1;

                    // go to next line
                    if self.width.is_some() && self.bg.is_some() {
                        for fx in cx..ex {
                            self.print_char(fx, cy, None, None);
                        }
                    }
                    widest = max(widest, cx - x);
                    cx = x;
                    cy += 1;
                    line_left = width;
                    // println!("- next line");

                    let len = self.print_word(cx, cy, right);
                    cx += len;
                    line_left -= len;
                    // println!("- add half: word={:?}, cx={}, ll={}", right, cx, line_left);
                } else {
                    // go to next line
                    if self.width.is_some() && self.bg.is_some() {
                        for fx in cx..ex {
                            self.print_char(fx, cy, None, None);
                        }
                    }
                    widest = max(widest, cx - x);
                    cx = x;
                    cy += 1;
                    line_left = width;
                    // println!("- next line");

                    let len = self.print_word(cx, cy, word);
                    cx += len;
                    line_left -= len;
                    // println!("- add word, cx={}, ll={}", cx, line_left);
                }
            }
        }
        if self.width.is_some() && self.bg.is_some() {
            for fx in cx..ex {
                self.print_char(fx, cy, None, None);
            }
        }
        widest = max(widest, cx - x);

        (widest, cy - y + 1)
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

type CharItem = (Option<RGBA>, char);
type CharVec = Vec<CharItem>;
type WordVec = Vec<CharVec>;
type LineVec = Vec<CharVec>;
type LinesOfWordsVec = Vec<WordVec>;

fn make_words(chars: CharVec) -> WordVec {
    let mut out: WordVec = Vec::new();
    out.push(Vec::new());

    for (fg, ch) in chars {
        if ch == ' ' {
            out.push(Vec::new());
        } else {
            out.last_mut().unwrap().push((fg, ch));
        }
    }

    out
}

fn make_lines(chars: CharVec) -> LineVec {
    let mut out: LineVec = Vec::new();
    out.push(Vec::new());

    for (fg, ch) in chars {
        if ch == '\n' {
            out.push(Vec::new());
        } else {
            out.last_mut().unwrap().push((fg, ch));
        }
    }
    out
}

fn make_lines_of_words(chars: CharVec) -> LinesOfWordsVec {
    let lines = make_lines(chars);

    let mut output: LinesOfWordsVec = Vec::new();

    for line in lines {
        let words = make_words(line);
        output.push(words);
    }

    output
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

    fn extract_line(buf: &Buffer, x: i32, y: i32, width: i32) -> String {
        let mut output = "".to_string();
        for cx in x..x + width {
            if let Some(g) = buf.get_glyph(cx, y) {
                output.push(char::from_u32(*g).unwrap());
            }
        }
        output
    }

    #[test]
    fn wrap_basic() {
        let mut buffer = Buffer::new(50, 50);
        let mut printer = colored(&mut buffer).width(10);

        assert_eq!(printer.wrap(0, 0, "taco casa"), (9, 1));
        assert_eq!(extract_line(&buffer, 0, 0, 10), "taco casa\0");
    }

    #[test]
    fn wrap_multi() {
        let mut buffer = Buffer::new(50, 50);
        let mut printer = colored(&mut buffer).width(10);

        let r = printer.wrap(0, 1, "#[red]taco casa#[] is a great fast food place");
        assert_eq!(extract_line(&buffer, 0, 1, 11), "taco casa\0\0");
        assert_eq!(extract_line(&buffer, 0, 2, 11), "is a great\0");
        assert_eq!(extract_line(&buffer, 0, 3, 11), "fast food\0\0");
        assert_eq!(extract_line(&buffer, 0, 4, 11), "place\0\0\0\0\0\0");
        assert_eq!(r, (10, 4));
    }

    #[test]
    fn wrap_breakword() {
        let mut buffer = Buffer::new(50, 50);
        let mut printer = colored(&mut buffer).width(10);

        let r = printer.wrap(0, 1, "supercalafragalisticexpialadocious");
        assert_eq!(extract_line(&buffer, 0, 1, 11), "supercala-\0");
        assert_eq!(extract_line(&buffer, 0, 2, 11), "fragalist-\0");
        assert_eq!(extract_line(&buffer, 0, 3, 11), "icexpiala-\0");
        assert_eq!(extract_line(&buffer, 0, 4, 11), "docious\0\0\0\0");
        assert_eq!(r, (10, 4));
    }

    #[test]
    fn wrap_multi_hyphen() {
        let mut buffer = Buffer::new(50, 50);
        let mut printer = colored(&mut buffer).width(10);

        let r = printer.wrap(
            0,
            1,
            "the conflaguration exponentially #[#f00]deteriorated#[] the stonemasons' monuments",
        );
        assert_eq!(extract_line(&buffer, 0, 1, 11), "the confl-\0");
        assert_eq!(extract_line(&buffer, 0, 2, 11), "aguration\0\0");
        assert_eq!(extract_line(&buffer, 0, 3, 11), "exponenti-\0");
        assert_eq!(extract_line(&buffer, 0, 4, 11), "ally dete-\0");
        assert_eq!(extract_line(&buffer, 0, 5, 11), "riorated\0\0\0");
        assert_eq!(extract_line(&buffer, 0, 6, 11), "the stone-\0");
        assert_eq!(extract_line(&buffer, 0, 7, 11), "masons'\0\0\0\0");
        assert_eq!(extract_line(&buffer, 0, 8, 11), "monuments\0\0");
        assert_eq!(r, (10, 8));
    }

    #[test]
    fn wrap_lines() {
        let mut buffer = Buffer::new(50, 50);
        let mut printer = colored(&mut buffer).width(20);

        let r = printer.wrap(
            0,
            1,
            "the conflaguration\nexponentially\ndeteriorated the\nstonemasons' monuments",
        );
        assert_eq!(extract_line(&buffer, 0, 1, 21), "the conflaguration\0\0\0");
        assert_eq!(
            extract_line(&buffer, 0, 2, 21),
            "exponentially\0\0\0\0\0\0\0\0"
        );
        assert_eq!(
            extract_line(&buffer, 0, 3, 21),
            "deteriorated the\0\0\0\0\0"
        );
        assert_eq!(extract_line(&buffer, 0, 4, 21), "stonemasons' monu-\0\0\0");
        assert_eq!(
            extract_line(&buffer, 0, 5, 21),
            "ments\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"
        );
        assert_eq!(r, (18, 5));
    }
}
