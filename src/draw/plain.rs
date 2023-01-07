use crate::simple::{Buffer, Glyph};
use crate::RGBA;
use std::cmp::{max, min};

#[derive(Copy, Clone)]
pub enum TextAlign {
    Left,
    Right,
    Center,
}

pub fn plain<'a>(buffer: &'a mut Buffer) -> PlainPrinter {
    PlainPrinter::new(buffer)
}

pub fn text<'a>(buffer: &'a mut Buffer) -> PlainPrinter {
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
            fg: Some(RGBA::rgb(255, 255, 255)),
            bg: None,
            to_glyph: &|ch| ch as u32,
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

    fn print_char(&mut self, x: i32, y: i32, ch: char) {
        let glyph = (self.to_glyph)(ch);
        self.buffer.draw_opt(x, y, Some(glyph), self.fg, self.bg);
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
        if ix + width > self.buffer.width() as i32 {
            width = self.buffer.width() as i32 - ix;
        }

        self.print_part(ix, y, start as usize, width as usize, text)
    }

    fn print_part(&mut self, x: i32, y: i32, start: usize, count: usize, text: &str) -> i32 {
        let mut chars = text.chars().skip(start);
        let mut ix = x;
        for _ in 0..count {
            let ch = match chars.next() {
                None => '\0',
                Some(ch) => ch,
            };
            self.print_char(ix, y, ch);
            // self.buffer.draw_opt(ix, y, glyph, self.fg, self.bg);
            ix += 1;
        }
        count as i32
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

    pub fn wrap(&mut self, x: i32, y: i32, text: &str) -> (i32, i32) {
        let width = self.width.unwrap_or(self.buffer.width() as i32 - x);

        let mut widest = 0;

        let mut cy = y;
        for line in wrap(width as usize, text) {
            let w = line.print(self, x, cy);
            widest = max(widest, w);
            cy += 1;
        }

        (widest, cy - y)

        // let mut cx = x;
        // let mut cy = y;
        // let mut line_left = width;
        // let ex = x + width;

        // // println!("==========================");
        // // println!("WRAP = {}", text);

        // for (i, line) in text.split('\n').enumerate() {
        //     if i > 0 {
        //         if self.width.is_some() && self.bg.is_some() {
        //             for fx in cx..ex {
        //                 self.print_char(fx, cy, '\0');
        //             }
        //         }
        //         widest = max(widest, cx - x);
        //         cx = x;
        //         cy += 1;
        //         line_left = width;
        //     }
        //     for (i, word) in line.split(' ').enumerate() {
        //         // println!(
        //         //     "word={}, len={}, cx={}, line_left={}",
        //         //     word,
        //         //     word.len(),
        //         //     cx,
        //         //     line_left
        //         // );
        //         let mut added_space = false;
        //         if i > 0 && line_left > word.len() as i32 {
        //             self.print_char(cx, cy, ' ');
        //             line_left -= 1;
        //             cx += 1;
        //             // println!("- add space, cx={}, ll={}", cx, line_left);
        //             added_space = true;
        //         }

        //         if word.len() == 0 {
        //             if line_left > 0 {
        //                 self.print_char(cx, cy, ' ');
        //                 line_left -= 1;
        //                 cx += 1;
        //                 // println!("- add space, cx={}, ll={}", cx, line_left);
        //             }
        //         } else if (word.len() as i32) <= line_left && (i == 0 || added_space) {
        //             let word_len = self.print_part(cx, cy, 0, word.len() as usize, word);
        //             cx += word_len;
        //             line_left -= word_len;
        //             // println!("- add word, cx={}, ll={}", cx, line_left);
        //         } else if (word.len() as i32) > width {
        //             // We are longer than a single line
        //             // Do we fit on this line and the next
        //             // println!("- long word");

        //             if line_left < 4 {
        //                 if self.width.is_some() && self.bg.is_some() {
        //                     for fx in cx..ex {
        //                         self.print_char(fx, cy, '\0');
        //                     }
        //                 }

        //                 widest = max(widest, cx - x);
        //                 cx = x;
        //                 cy += 1;
        //                 line_left = width;
        //                 // println!("- push to next line");
        //             } else if cx > x {
        //                 self.print_char(cx, cy, ' ');
        //                 line_left -= 1;
        //                 cx += 1;
        //                 // println!("- space");
        //             }

        //             for ch in word.chars() {
        //                 if line_left == 1 {
        //                     self.print_char(cx, cy, '-');
        //                     widest = max(widest, cx - x + 1);
        //                     cx = x;
        //                     line_left = width;
        //                     cy += 1;
        //                     // println!("- hyphen + next line");
        //                 }

        //                 self.print_char(cx, cy, ch);
        //                 line_left -= 1;
        //                 cx += 1;
        //                 // println!("- add letter, ch={}, cx={}, ll={}", ch, cx, line_left);
        //             }
        //         } else if word.len() > 6 && line_left - 2 >= word.len() as i32 / 2 {
        //             let pivot = min(line_left - 2, word.len() as i32 / 2);

        //             let left = &word[..pivot as usize];
        //             let right = &word[pivot as usize..];

        //             if cx > x {
        //                 self.print_char(cx, cy, ' ');
        //                 // line_left -= 1;
        //                 cx += 1;
        //                 // println!("- space");
        //             }

        //             let len = self.print_part(cx, cy, 0, left.len(), left);
        //             cx += len;
        //             // line_left -= len;
        //             // println!("- add half: word={}, cx={}, ll={}", left, cx, line_left);
        //             self.print_char(cx, cy, '-');
        //             cx += 1;

        //             // go to next line
        //             if self.width.is_some() && self.bg.is_some() {
        //                 for fx in cx..ex {
        //                     self.print_char(fx, cy, '\0');
        //                 }
        //             }
        //             widest = max(widest, cx - x);
        //             cx = x;
        //             cy += 1;
        //             line_left = width;
        //             // println!("- next line");

        //             let len = self.print_part(cx, cy, 0, right.len(), right);
        //             cx += len;
        //             line_left -= len;
        //             // println!("- add half: word={}, cx={}, ll={}", right, cx, line_left);
        //         } else {
        //             // go to next line
        //             if self.width.is_some() && self.bg.is_some() {
        //                 for fx in cx..ex {
        //                     self.print_char(fx, cy, '\0');
        //                 }
        //             }
        //             widest = max(widest, cx - x);
        //             cx = x;
        //             cy += 1;
        //             line_left = width;
        //             // println!("- next line");

        //             let len = self.print_part(cx, cy, 0, word.len(), word);
        //             cx += len;
        //             line_left -= len;
        //             // println!("- add word, cx={}, ll={}", cx, line_left);
        //         }
        //     }
        // }

        // if self.width.is_some() && self.bg.is_some() {
        //     for fx in cx..ex {
        //         self.print_char(fx, cy, '\0');
        //     }
        // }
        // widest = max(widest, cx - x);

        // (widest, cy - y + 1)
    }
}

struct Line<'a>(&'a str, bool);

impl<'a> Line<'a> {
    pub fn new(t: &'a str) -> Self {
        Line(t, false)
    }

    // pub fn len(&self) -> usize {
    //     self.0.chars().count() + if self.1 { 1 } else { 0 }
    // }

    pub fn with_hyphen(mut self) -> Self {
        self.1 = true;
        self
    }

    pub fn char_len(&self) -> usize {
        self.0.chars().count() + if self.1 { 1 } else { 0 }
    }

    pub fn last_break_before(&self, char_idx: usize) -> Option<usize> {
        let idx = self.0.char_indices().nth(char_idx).map(|(i, _)| i).unwrap();
        match self.0[..idx].rmatch_indices(' ').next() {
            None => None,
            Some((idx, _)) => Some(idx),
        }
    }

    pub fn first_word(&self) -> Self {
        match self.0.find(" ") {
            None => Line::new(self.0),
            Some(idx) => Line::new(&self.0[..idx]),
        }
    }

    // pub fn left(&self, len: usize) -> Self {
    //     Line::new(&self.0[..len])
    // }

    pub fn hyphenate_at_char(&self, char_idx: usize) -> (Self, Self) {
        let idx = self.0.char_indices().nth(char_idx).map(|(i, _)| i).unwrap();
        (
            Line::new(&self.0[..idx]).with_hyphen(),
            Line::new(&self.0[idx..]),
        )
    }

    pub fn split_at_space(&self, char_idx: usize) -> (Self, Self) {
        let idx = self.0.char_indices().nth(char_idx).map(|(i, _)| i).unwrap();
        (Line::new(&self.0[..idx]), Line::new(&self.0[idx + 1..]))
    }

    pub fn print(&self, printer: &mut PlainPrinter, x: i32, y: i32) -> i32 {
        let width = printer.width.unwrap_or(self.char_len() as i32);
        let self_len = min(width, self.char_len() as i32);
        let spaces = width.saturating_sub(self_len);

        let (x, pre, post) = match printer.align {
            TextAlign::Left => (x, 0, spaces),
            TextAlign::Center => {
                let half = spaces / 2;
                (x - half - self_len / 2, half, spaces - half)
            }
            TextAlign::Right => (x - width + 1, spaces, 0),
        };

        let mut cx = x;
        let fg = printer.fg;
        let bg = printer.bg;

        // let mut output = "[".to_string();
        for _ in 0..pre {
            printer.buffer.draw_opt(cx, y, Some(0), fg, bg);
            cx += 1;
        }

        // output += self.0;
        for char in self.0.chars() {
            let glyph = (printer.to_glyph)(char);
            printer.buffer.draw_opt(cx, y, Some(glyph), fg, bg);
            cx += 1;
        }

        if self.1 {
            printer.buffer.draw_opt(cx, y, Some('-' as u32), fg, bg);
            cx += 1;
        }

        for _ in 0..post {
            printer.buffer.draw_opt(cx, y, Some(0), fg, bg);
            cx += 1;
        }

        // output.push(']');

        // println!("{} [{}]", output, output.len() - 2);
        width
    }
}

fn wrap<'a>(limit: usize, text: &'a str) -> Vec<Line<'a>> {
    println!("--------------------------------------");
    println!("WRAP - {}: '{}'", limit, text);

    let mut output: Vec<Line<'a>> = Vec::new();

    for line in text.split('\n') {
        let mut current = Line::new(line);
        let mut i = 0;

        while current.char_len() > limit {
            i += 1;
            if i > 10 {
                break;
            }

            match current.last_break_before(limit + 1) {
                None => {
                    let first_word = current.first_word();
                    let first_word_len = first_word.char_len();

                    println!("too long - {}", first_word.0);

                    let keep_len = min(limit - 1, first_word_len - 2);
                    let (left, right) = current.hyphenate_at_char(keep_len);
                    output.push(left);
                    current = right;
                }
                Some(break_index) => {
                    let (mut left, mut right) = current.split_at_space(break_index);
                    let left_len = left.char_len();
                    let line_left = limit.saturating_sub(left_len).saturating_sub(1);

                    println!(
                        " - left={}, line_left={}, right={}",
                        left.0, line_left, right.0
                    );
                    if line_left >= 4 {
                        let next_word = right.first_word();
                        let next_word_len = next_word.char_len();

                        println!(" - : next_word={}, len={}", next_word.0, next_word_len);

                        if next_word_len >= 6 {
                            let keep_len = min(line_left, next_word_len - 2);
                            println!(" - : hyphen! keep={}", keep_len);
                            (left, right) = current.hyphenate_at_char(break_index + keep_len);
                        }
                    }
                    output.push(left);
                    current = right;
                }
            }
        }

        if current.char_len() > 0 {
            output.push(current);
        }
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;

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
        let mut printer = plain(&mut buffer).width(10);

        assert_eq!(printer.wrap(0, 0, "taco casa"), (10, 1));
        assert_eq!(extract_line(&buffer, 0, 0, 10), "taco casa\0");
    }

    #[test]
    fn wrap_multi() {
        let mut buffer = Buffer::new(50, 50);
        let mut printer = plain(&mut buffer).width(10);

        let r = printer.wrap(0, 1, "taco casa is a great fast food place");
        assert_eq!(extract_line(&buffer, 0, 1, 11), "taco casa\0\0");
        assert_eq!(extract_line(&buffer, 0, 2, 11), "is a great\0");
        assert_eq!(extract_line(&buffer, 0, 3, 11), "fast food\0\0");
        assert_eq!(extract_line(&buffer, 0, 4, 11), "place\0\0\0\0\0\0");
        assert_eq!(r, (10, 4));
    }

    #[test]
    fn wrap_breakword() {
        let mut buffer = Buffer::new(50, 50);
        let mut printer = plain(&mut buffer).width(10);

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
        let mut printer = plain(&mut buffer).width(10);

        let r = printer.wrap(
            0,
            1,
            "the conflaguration exponentially deteriorated the stonemasons' monuments",
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
        let mut printer = plain(&mut buffer).width(20);

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
        assert_eq!(extract_line(&buffer, 0, 4, 21), "stonemasons' monume-\0");
        assert_eq!(
            extract_line(&buffer, 0, 5, 21),
            "nts\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"
        );
        assert_eq!(r, (20, 5));
    }

    #[test]
    fn wrap_width() {
        let mut buffer = Buffer::new(50, 50);
        {
            let mut printer = plain(&mut buffer).width(15);

            let r = printer.wrap(0, 0, "Inside a call to wrap, you can place a long text and it will automatically be wrapped at the width you specify.");

            assert_eq!(extract_line(&buffer, 0, 0, 16), "Inside a call\0\0\0");
            assert_eq!(extract_line(&buffer, 0, 1, 16), "to wrap, you\0\0\0\0");
            assert_eq!(extract_line(&buffer, 0, 2, 16), "can place a\0\0\0\0\0");
            assert_eq!(extract_line(&buffer, 0, 3, 16), "long text and\0\0\0");
            assert_eq!(extract_line(&buffer, 0, 4, 16), "it will automa-\0");
            assert_eq!(extract_line(&buffer, 0, 5, 16), "tically be wra-\0");
            assert_eq!(extract_line(&buffer, 0, 6, 16), "pped at the\0\0\0\0\0");
            assert_eq!(extract_line(&buffer, 0, 7, 16), "width you spec-\0");
            assert_eq!(
                extract_line(&buffer, 0, 8, 16),
                "ify.\0\0\0\0\0\0\0\0\0\0\0\0"
            );
            assert_eq!(r, (15, 9));
        }

        {
            let mut printer = plain(&mut buffer).width(15);

            let r = printer.wrap(0, 0, "Inside a call to wrap, you can place a long text and it will automatically be wrapped at the width you specify.");

            assert_eq!(extract_line(&buffer, 0, 0, 16), "Inside a call\0\0\0");
            assert_eq!(extract_line(&buffer, 0, 1, 16), "to wrap, you\0\0\0\0");
            assert_eq!(extract_line(&buffer, 0, 2, 16), "can place a\0\0\0\0\0");
            assert_eq!(extract_line(&buffer, 0, 3, 16), "long text and\0\0\0");
            assert_eq!(extract_line(&buffer, 0, 4, 16), "it will automa-\0");
            assert_eq!(extract_line(&buffer, 0, 5, 16), "tically be wra-\0");
            assert_eq!(extract_line(&buffer, 0, 6, 16), "pped at the\0\0\0\0\0");
            assert_eq!(extract_line(&buffer, 0, 7, 16), "width you spec-\0");
            assert_eq!(
                extract_line(&buffer, 0, 8, 16),
                "ify.\0\0\0\0\0\0\0\0\0\0\0\0"
            );
            assert_eq!(r, (15, 9));
        }
    }
}
