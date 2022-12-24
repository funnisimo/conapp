use super::TextAlign;
use crate::codepage437;
use crate::Buffer;
use crate::Glyph;
use crate::{to_rgba, RGBA};
use std::cmp::{max, min};
use std::fmt::Display;

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

    // fn print_char(&mut self, x: i32, y: i32, ch: Option<char>, fg: Option<RGBA>) {
    //     let glyph = match ch {
    //         None => Some(0),
    //         Some(ch) => Some((self.to_glyph)(ch)),
    //     };
    //     self.buffer.draw_opt(x, y, glyph, fg.or(self.fg), self.bg);
    // }

    pub fn print(&mut self, x: i32, y: i32, text: &str) -> i32 {
        // let width = self.width.unwrap_or(self.buffer.get_width() as i32 - x);
        let mut widest = 0;

        let mut cy = y;
        for line in parse_colored_lines(text).iter().take(1) {
            let w = line.print(self, x, cy);
            widest = max(widest, w);
            cy += 1;
        }

        widest
    }

    // fn print_line(&mut self, x: i32, y: i32, chars: &[(Option<RGBA>, char)]) -> i32 {
    //     let mut width = self.width.unwrap_or(chars.len() as i32);
    //     let mut start = 0;
    //     let mut ix = match self.align {
    //         TextAlign::Left => x,
    //         TextAlign::Right => x - width + 1,
    //         TextAlign::Center => x - width / 2,
    //     };
    //     if ix < 0 {
    //         width += ix;
    //         start -= ix;
    //         ix = 0;
    //     }
    //     if ix + width > self.buffer.get_width() as i32 {
    //         width = self.buffer.get_width() as i32 - ix;
    //     }
    //     let mut iter = chars.iter().skip(start as usize);
    //     for _ in 0..width {
    //         let (fg, ch) = match iter.next() {
    //             None => (None, None),
    //             Some(x) => (x.0, Some(x.1)),
    //         };
    //         self.print_char(ix, y, ch, fg);
    //         ix += 1;
    //     }
    //     width
    // }

    pub fn print_lines(&mut self, x: i32, y: i32, text: &str) -> (i32, i32) {
        // let width = self.width.unwrap_or(self.buffer.get_width() as i32 - x);

        let mut widest = 0;

        let mut cy = y;
        for line in parse_colored_lines(text) {
            let w = line.print(self, x, cy);
            widest = max(widest, w);
            cy += 1;
        }

        (widest, cy - y)

        // let chars: Vec<(Option<RGBA>, char)> = TextIterator::new(self.to_rgba, text).collect();

        // let lines = chars
        //     .into_iter()
        //     .fold(vec![Vec::new()], |mut acc, (color, ch)| {
        //         if ch == '\n' {
        //             acc.push(Vec::new());
        //         } else {
        //             acc.last_mut().unwrap().push((color, ch));
        //         }
        //         acc
        //     });

        // let mut width = 0;
        // let mut height = 0;
        // for line in lines {
        //     let w = self.print_line(x, y + height, &line);
        //     width = max(width, w);
        //     height += 1;
        // }
        // (width, height)
    }

    // fn print_word(&mut self, x: i32, y: i32, chars: &[(Option<RGBA>, char)]) -> i32 {
    //     let width = chars.len() as i32;
    //     let mut iter = chars.iter();
    //     let mut ix = x;
    //     for _ in 0..width {
    //         let (fg, ch) = match iter.next() {
    //             None => (None, None),
    //             Some(x) => (x.0, Some(x.1)),
    //         };
    //         self.print_char(ix, y, ch, fg);
    //         ix += 1;
    //     }
    //     width
    // }

    pub fn wrap(&mut self, x: i32, y: i32, text: &str) -> (i32, i32) {
        let width = self.width.unwrap_or(self.buffer.get_width() as i32 - x);

        let mut widest = 0;

        let mut cy = y;
        for line in wrap(width as usize, text) {
            let w = line.print(self, x, cy);
            widest = max(widest, w);
            cy += 1;
        }

        (widest, cy - y)

        // let width = self.width.unwrap_or(self.buffer.get_width() as i32 - x);

        // let chars: Vec<(Option<RGBA>, char)> = TextIterator::new(self.to_rgba, text).collect();

        // let lines_of_words = make_lines_of_words(chars);

        // // println!("==========================");
        // // println!("WRAP = {}", text);

        // let mut widest = 0;
        // let mut cx = x;
        // let mut cy = y;
        // let mut line_left = width;
        // let ex = x + width;

        // for (i, line) in lines_of_words.iter().enumerate() {
        //     if i > 0 {
        //         if self.width.is_some() && self.bg.is_some() {
        //             for fx in cx..ex {
        //                 self.print_char(fx, cy, None, None);
        //             }
        //         }
        //         widest = max(widest, cx - x);
        //         cx = x;
        //         cy += 1;
        //         line_left = width;
        //     }

        //     for (i, word) in line.iter().enumerate() {
        //         // println!(
        //         //     "word={:?}, len={}, cx={}, line_left={}",
        //         //     word,
        //         //     word.len(),
        //         //     cx,
        //         //     line_left
        //         // );
        //         let first_fg = word.first().unwrap_or(&(None, ' ')).0;

        //         if i > 0 && line_left > word.len() as i32 {
        //             self.print_char(cx, cy, Some(' '), first_fg);
        //             line_left -= 1;
        //             cx += 1;
        //             // println!("- add space, cx={}, ll={}", cx, line_left);
        //         }

        //         if word.len() == 0 {
        //             if line_left > 0 {
        //                 self.print_char(cx, cy, Some(' '), first_fg);
        //                 line_left -= 1;
        //                 cx += 1;
        //                 // println!("- add space, cx={}, ll={}", cx, line_left);
        //             }
        //         } else if (word.len() as i32) <= line_left {
        //             let word_len = self.print_word(cx, cy, word);
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
        //                         self.print_char(fx, cy, None, None);
        //                     }
        //                 }
        //                 widest = max(widest, cx - x);
        //                 cx = x;
        //                 cy += 1;
        //                 line_left = width;
        //                 // println!("- push to next line");
        //             } else if cx > x {
        //                 self.print_char(cx, cy, Some(' '), first_fg);
        //                 line_left -= 1;
        //                 cx += 1;
        //                 // println!("- space");
        //             }

        //             for (fg, ch) in word {
        //                 if line_left == 1 {
        //                     self.print_char(cx, cy, Some('-'), *fg);
        //                     cx += 1;

        //                     if self.width.is_some() && self.bg.is_some() {
        //                         for fx in cx..ex {
        //                             self.print_char(fx, cy, None, None);
        //                         }
        //                     }

        //                     widest = max(widest, cx - x);
        //                     cx = x;
        //                     line_left = width;
        //                     cy += 1;
        //                     // println!("- hyphen + next line");
        //                 }

        //                 self.print_char(cx, cy, Some(*ch), *fg);
        //                 line_left -= 1;
        //                 cx += 1;
        //                 // println!("- add letter, ch={}, cx={}, ll={}", ch, cx, line_left);
        //             }
        //         } else if word.len() > 6 && line_left - 2 >= word.len() as i32 / 2 {
        //             let pivot = min(line_left - 2, word.len() as i32 / 2);

        //             let left = &word[..pivot as usize];
        //             let right = &word[pivot as usize..];

        //             if cx > x {
        //                 self.print_char(cx, cy, Some(' '), first_fg);
        //                 // line_left -= 1;
        //                 cx += 1;
        //                 // println!("- space");
        //             }

        //             let len = self.print_word(cx, cy, left);
        //             cx += len;
        //             // line_left -= len;
        //             // println!("- add half: word={:?}, cx={}, ll={}", left, cx, line_left);
        //             self.print_char(cx, cy, Some('-'), first_fg);
        //             cx += 1;

        //             // go to next line
        //             if self.width.is_some() && self.bg.is_some() {
        //                 for fx in cx..ex {
        //                     self.print_char(fx, cy, None, None);
        //                 }
        //             }
        //             widest = max(widest, cx - x);
        //             cx = x;
        //             cy += 1;
        //             line_left = width;
        //             // println!("- next line");

        //             let len = self.print_word(cx, cy, right);
        //             cx += len;
        //             line_left -= len;
        //             // println!("- add half: word={:?}, cx={}, ll={}", right, cx, line_left);
        //         } else {
        //             // go to next line
        //             if self.width.is_some() && self.bg.is_some() {
        //                 for fx in cx..ex {
        //                     self.print_char(fx, cy, None, None);
        //                 }
        //             }
        //             widest = max(widest, cx - x);
        //             cx = x;
        //             cy += 1;
        //             line_left = width;
        //             // println!("- next line");

        //             let len = self.print_word(cx, cy, word);
        //             cx += len;
        //             line_left -= len;
        //             // println!("- add word, cx={}, ll={}", cx, line_left);
        //         }
        //     }
        // }
        // if self.width.is_some() && self.bg.is_some() {
        //     for fx in cx..ex {
        //         self.print_char(fx, cy, None, None);
        //     }
        // }
        // widest = max(widest, cx - x);

        // (widest, cy - y + 1)
    }
}

#[derive(Debug, Clone)]
struct ColoredSpan<'a> {
    color: Option<&'a str>,
    txt: &'a str,
}

impl<'a> ColoredSpan<'a> {
    fn new(color: Option<&'a str>, txt: &'a str) -> Self {
        ColoredSpan { color, txt }
    }

    // pub fn start_trimmed(&self) -> Self {
    //     ColoredSpan::new(self.color, self.txt.trim_start())
    // }

    pub fn char_len(&self) -> usize {
        self.txt.chars().count()
    }

    pub fn last_break_before(&self, char_idx: usize) -> Option<usize> {
        if char_idx == 0 {
            return None;
        }
        match self.txt.char_indices().nth(char_idx) {
            // we only get this if are past the end of the slice
            None => match self.txt.rmatch_indices(' ').next() {
                None => None,
                Some((idx, _)) => Some(idx),
            },
            Some((idx, _)) => match self.txt[..idx].rmatch_indices(' ').next() {
                None => None,
                Some((idx, _)) => Some(idx),
            },
        }
    }

    // pub fn first_word(&self) -> Self {
    //     match self.txt.find(" ") {
    //         None => self.clone(),
    //         Some(idx) => ColoredSpan::new(self.color, &self.txt[..idx]),
    //     }
    // }

    pub fn split_at_char(&self, char_idx: usize) -> (Self, Self) {
        let idx = self
            .txt
            .char_indices()
            .nth(char_idx)
            .map(|(i, _)| i)
            .unwrap();
        (
            ColoredSpan::new(self.color, &self.txt[..idx]),
            ColoredSpan::new(self.color, &self.txt[idx..]),
        )
    }

    pub fn split_at_space(&self, omit_idx: usize) -> (Self, Self) {
        let idx = self
            .txt
            .char_indices()
            .nth(omit_idx)
            .map(|(i, _)| i)
            .unwrap();
        (
            ColoredSpan::new(self.color, &self.txt[..idx]),
            ColoredSpan::new(self.color, &self.txt[idx + 1..]),
        )
    }

    /// just print the text - nothing more, nothing less
    /// decisions about padding, alignment, etc... need to be in ColoredLine::print
    pub fn print(&self, printer: &mut ColoredPrinter, x: i32, y: i32) -> i32 {
        let mut cx = x;
        let fg = match self.color {
            None => printer.fg,
            Some(txt) => (printer.to_rgba)(txt),
        };
        let bg = printer.bg;

        for char in self.txt.chars() {
            let glyph = (printer.to_glyph)(char);
            printer.buffer.draw_opt(cx, y, Some(glyph), fg, bg);
            cx += 1;
        }

        cx - x
    }
}

impl<'a> Display for ColoredSpan<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            None => write!(f, "#[]{}", self.txt),
            Some(c) => write!(f, "#[{}]{}", c, self.txt),
        }
    }
}

#[derive(Debug, Clone)]
struct ColoredLine<'a> {
    spans: Vec<ColoredSpan<'a>>,
    // hyphen: bool,
}

impl<'a> ColoredLine<'a> {
    fn new() -> Self {
        ColoredLine {
            spans: Vec::new(),
            // hyphen: false,
        }
    }

    fn push(&mut self, span: ColoredSpan<'a>) {
        self.spans.push(span);
    }

    // pub fn new(txt: &'a str) -> Self {
    //     Line(txt,false)
    // }

    // pub fn trimmed(&self) -> Self {
    //     Line(self.0.trim(), self.1)
    // }

    // pub fn with_hyphen(mut self) -> Self {
    //     self.hyphen = true;
    //     self
    // }

    pub fn char_len(&self) -> usize {
        self.spans.iter().fold(0, |cnt, spn| cnt + spn.char_len())
    }

    pub fn last_break_before(&self, char_idx: usize) -> Option<usize> {
        //     match self.0[..idx].rmatch_indices(' ').next() {
        //         None => None,
        //         Some((idx,_)) => Some(idx)
        //     }

        println!("lbb - {}, {}", self, char_idx);

        let mut len_left = char_idx;
        let mut len_so_far = 0;
        let mut best: Option<usize> = None;

        for span in self.spans.iter() {
            if len_left == 0 {
                break;
            }
            let char_len = span.char_len();
            let my_max = min(char_len + 1, len_left);

            println!(" - span.lbb {}, {}", span, my_max);
            match span.last_break_before(my_max) {
                None => {}
                Some(idx) => {
                    println!(" - new best={}", len_so_far + idx);
                    best = Some(len_so_far + idx);
                }
            }
            len_left = len_left.saturating_sub(char_len);
            len_so_far += char_len;
        }

        println!(" : result={:?}", best);
        best
    }

    pub fn first_word(&self) -> Self {
        // match self.0.find(" ") {
        //     None => Line::new(self.0),
        //     Some(idx) => Line::new(&self.0[..idx]),
        // }
        let mut out = ColoredLine::new();
        for span in self.spans.iter() {
            match span.txt.find(" ") {
                None => out.push(span.clone()),
                Some(idx) => {
                    out.push(ColoredSpan::new(span.color, &span.txt[..idx]));
                    break;
                }
            }
        }
        out
    }

    pub fn hyphenate_at_char(&self, split_idx: usize) -> (Self, Self) {
        // let idx = self.0.char_indices().nth(char_idx).map(|(i,_)| i).unwrap();
        // (Line::new(&self.0[..idx]).with_hyphen(), Line::new(&self.0[idx..]))

        let mut left = ColoredLine::new();
        let mut right = ColoredLine::new();
        let mut len_so_far = 0;

        for span in self.spans.iter() {
            if len_so_far >= split_idx {
                right.spans.push(span.clone());
            } else {
                let char_len = span.char_len();
                if len_so_far + char_len == split_idx {
                    left.spans.push(span.clone());
                } else if len_so_far + char_len > split_idx {
                    let idx = split_idx - len_so_far;
                    let (a, b) = span.split_at_char(idx);
                    println!("hac - sac - {} = {:?} + {:?}", idx, a, b);
                    left.push(a);
                    left.push(ColoredSpan::new(span.color, "-"));
                    right.push(b);
                } else {
                    left.spans.push(span.clone());
                }
                len_so_far += char_len;
            }
        }

        (left, right)
    }

    pub fn split_at_space(&self, omit_idx: usize) -> (Self, Self) {
        //     let idx = self.0.char_indices().nth(char_idx).map(|(i,_)| i).unwrap();
        //     (Line::new(&self.0[..idx]), Line::new(&self.0[idx+1..]))
        let mut left = ColoredLine::new();
        let mut right = ColoredLine::new();
        let mut to_omit = omit_idx as i32;

        for span in self.spans.iter() {
            if to_omit < 0 {
                right.spans.push(span.clone());
            } else {
                let char_len = span.char_len() as i32;
                if to_omit < char_len {
                    let (a, b) = span.split_at_space(to_omit as usize);
                    left.push(a);
                    right.push(b);
                } else {
                    left.spans.push(span.clone());
                }
                to_omit -= char_len;
            }
        }

        (left, right)
    }

    pub fn print(&self, printer: &mut ColoredPrinter, x: i32, y: i32) -> i32 {
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
        for span in self.spans.iter() {
            let w = span.print(printer, cx, y);
            cx += w;
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

impl<'a> Display for ColoredLine<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for span in self.spans.iter() {
            match span.color {
                None => write!(f, "#[]{}", span.txt)?,
                Some(c) => write!(f, "#[{}]{}", c, span.txt)?,
            }
        }
        Ok(())
    }
}

fn parse_colored_lines<'a>(txt: &'a str) -> Vec<ColoredLine<'a>> {
    let mut colors: Vec<Option<&str>> = Vec::new();
    let mut out: Vec<ColoredLine<'a>> = Vec::new();

    for line in txt.split('\n') {
        let colored_line = parse_colored_line(line, &mut colors);
        out.push(colored_line);
    }

    // println!("- {:?}", out);
    // println!("--");
    out
}

fn parse_colored_line<'a>(line: &'a str, colors: &mut Vec<Option<&'a str>>) -> ColoredLine<'a> {
    let mut colored_line = ColoredLine::new();
    let default_color: Option<&str> = None;

    for (i, major_part) in line.split("#[").enumerate() {
        if major_part.len() == 0 {
            continue;
        } // skip empty parts
        if i == 0 {
            colored_line.push(ColoredSpan::new(default_color, major_part));
        } else if major_part.starts_with("[") {
            let c = colors.iter().last().unwrap_or(&default_color);
            colored_line.push(ColoredSpan::new(c.clone(), "#["));
            colored_line.push(ColoredSpan::new(c.clone(), &major_part[1..]));
        } else {
            match major_part.split_once("]") {
                None => panic!("Parsing error! - {}", line),
                Some((color, text)) => {
                    if color.len() == 0 {
                        colors.pop();
                    } else {
                        colors.push(Some(color));
                    }
                    let c = colors.iter().last().unwrap_or(&default_color);
                    colored_line.push(ColoredSpan::new(c.clone(), text));
                }
            }
        }
    }

    colored_line
}

fn wrap<'a>(limit: usize, text: &'a str) -> Vec<ColoredLine<'a>> {
    println!("--------------------------------------");
    println!("WRAP - {}: '{}'", limit, text);

    let mut output: Vec<ColoredLine<'a>> = Vec::new();

    for mut current in parse_colored_lines(text) {
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

                    let keep_len = min(limit - 1, first_word_len - 2);
                    let (left, right) = current.hyphenate_at_char(keep_len);

                    println!("too long - {} => {} + {}", first_word, left, right);

                    println!(": {}", left);
                    output.push(left);
                    current = right;
                }
                Some(break_index) => {
                    let (mut left, mut right) = current.split_at_space(break_index);
                    let left_len = left.char_len();
                    let line_left = limit.saturating_sub(left_len).saturating_sub(1);

                    println!(" - left={}, line_left={}, right={}", left, line_left, right);
                    if line_left >= 4 {
                        let next_word = right.first_word();
                        let next_word_len = next_word.char_len();

                        println!(" - : next_word={}, len={}", next_word, next_word_len);

                        if next_word_len >= 6 {
                            let keep_len = min(line_left, next_word_len - 2);
                            println!(" - : hyphen! keep={}", keep_len);
                            (left, right) = current.hyphenate_at_char(break_index + keep_len);
                        }
                    }
                    println!(": {}", left);
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

    const _WHITE: RGBA = RGBA::rgb(255, 255, 255);
    const _RED: RGBA = RGBA::rgb(255, 0, 0);
    const _GREEN: RGBA = RGBA::rgb(0, 255, 0);
    const _BLUE: RGBA = RGBA::rgb(0, 0, 255);
    const _BLACK: RGBA = RGBA::rgb(0, 0, 0);

    // #[test]
    // fn no_color() {
    //     let mut iter = TextIterator::new(&|_| Some(WHITE), "Text");

    //     assert_eq!(iter.next().unwrap(), (None, 'T'));
    //     assert_eq!(iter.next().unwrap(), (None, 'e'));
    //     assert_eq!(iter.next().unwrap(), (None, 'x'));
    //     assert_eq!(iter.next().unwrap(), (None, 't'));
    //     assert_eq!(iter.next(), None);
    // }

    // #[test]
    // fn start_color() {
    //     fn to_rgba(_: &str) -> Option<RGBA> {
    //         Some(BLUE)
    //     }

    //     let mut iter = TextIterator::new(&to_rgba, "#[blue]Text");

    //     assert_eq!(iter.next().unwrap(), (Some(BLUE), 'T'));
    //     assert_eq!(iter.next().unwrap(), (Some(BLUE), 'e'));
    //     assert_eq!(iter.next().unwrap(), (Some(BLUE), 'x'));
    //     assert_eq!(iter.next().unwrap(), (Some(BLUE), 't'));
    //     assert_eq!(iter.next(), None);
    // }

    // #[test]
    // fn mid_color() {
    //     fn to_rgba(t: &str) -> Option<RGBA> {
    //         match t {
    //             "blue" => Some(BLUE),
    //             "white" => Some(WHITE),
    //             _ => None,
    //         }
    //     }

    //     let mut iter = TextIterator::new(&to_rgba, "a #[blue]b#[] c");

    //     assert_eq!(iter.next().unwrap(), (None, 'a'));
    //     assert_eq!(iter.next().unwrap(), (None, ' '));
    //     assert_eq!(iter.next().unwrap(), (Some(BLUE), 'b'));
    //     assert_eq!(iter.next().unwrap(), (None, ' '));
    //     assert_eq!(iter.next().unwrap(), (None, 'c'));
    //     assert_eq!(iter.next(), None);
    // }

    // #[test]
    // fn escape_color() {
    //     let mut iter = TextIterator::new(&|_| Some(RED), "a #[[blue]b#[[] c");

    //     assert_eq!(iter.next().unwrap(), (None, 'a'));
    //     assert_eq!(iter.next().unwrap(), (None, ' '));
    //     assert_eq!(iter.next().unwrap(), (None, '#'));
    //     assert_eq!(iter.next().unwrap(), (None, '['));
    //     assert_eq!(iter.next().unwrap(), (None, 'b'));
    //     assert_eq!(iter.next().unwrap(), (None, 'l'));
    //     assert_eq!(iter.next().unwrap(), (None, 'u'));
    //     assert_eq!(iter.next().unwrap(), (None, 'e'));
    //     assert_eq!(iter.next().unwrap(), (None, ']'));
    //     assert_eq!(iter.next().unwrap(), (None, 'b'));
    //     assert_eq!(iter.next().unwrap(), (None, '#'));
    //     assert_eq!(iter.next().unwrap(), (None, '['));
    //     assert_eq!(iter.next().unwrap(), (None, ']'));
    //     assert_eq!(iter.next().unwrap(), (None, ' '));
    //     assert_eq!(iter.next().unwrap(), (None, 'c'));
    //     assert_eq!(iter.next(), None);
    // }

    #[test]
    fn span_last_break_before() {
        let text = "This is a span of text";
        let span = ColoredSpan::new(Some("color"), text);

        assert_eq!(span.last_break_before(0), None);
        assert_eq!(span.last_break_before(4), None);
        assert_eq!(span.last_break_before(5), Some(4));
        assert_eq!(span.last_break_before(12), Some(9));
        assert_eq!(span.last_break_before(20), Some(17));

        let text = "This is a ";
        let span = ColoredSpan::new(Some("color"), text);

        assert_eq!(span.last_break_before(0), None);
        assert_eq!(span.last_break_before(0), None);
        assert_eq!(span.last_break_before(4), None);
        assert_eq!(span.last_break_before(5), Some(4));
        assert_eq!(span.last_break_before(12), Some(9));
    }

    #[test]
    fn line_last_break_before() {
        let text = "This is a #[00F]span#[] of text";
        let mut colors = Vec::new();
        let line = parse_colored_line(text, &mut colors);
        // let mut buffer = Buffer::new(50, 50);

        assert_eq!(line.last_break_before(0), None);
        assert_eq!(line.last_break_before(4), None);
        assert_eq!(line.last_break_before(5), Some(4));
        assert_eq!(line.last_break_before(12), Some(9));
        assert_eq!(line.last_break_before(20), Some(17));
    }

    #[test]
    fn span_split_at_space() {
        let text = "This is a span of text";
        let span = ColoredSpan::new(Some("color"), text);

        let (left, right) = span.split_at_space(9);
        assert_eq!(left.txt, "This is a");
        assert_eq!(right.txt, "span of text");
        assert_eq!(left.color, Some("color"));
        assert_eq!(right.color, Some("color"));

        let (left, right) = span.split_at_space(0);
        assert_eq!(left.txt, "");
        assert_eq!(right.txt, "his is a span of text");
        assert_eq!(left.color, Some("color"));
        assert_eq!(right.color, Some("color"));

        let (left, right) = span.split_at_space(span.char_len() - 1);
        assert_eq!(left.txt, "This is a span of tex");
        assert_eq!(right.txt, "");
        assert_eq!(left.color, Some("color"));
        assert_eq!(right.color, Some("color"));
    }

    #[test]
    fn line_split_at_space() {
        let text = "This is a #[00F]span#[] of text";
        let mut colors = Vec::new();
        let line = parse_colored_line(text, &mut colors);
        let mut buffer = Buffer::new(50, 50);

        let (left, right) = line.split_at_space(9);
        {
            let mut printer = colored(&mut buffer);
            left.print(&mut printer, 0, 0);
            right.print(&mut printer, 0, 1);
        }
        assert_eq!(extract_line(&buffer, 0, 0, 10), "This is a\0");
        assert_eq!(extract_line(&buffer, 0, 1, 13), "span of text\0");

        buffer.clear(true, true, true);
        let (left, right) = line.split_at_space(0);
        {
            println!("left={:?}, right={:?}", left, right);
            let mut printer = colored(&mut buffer);
            left.print(&mut printer, 0, 0);
            right.print(&mut printer, 0, 1);
        }
        assert_eq!(extract_line(&buffer, 0, 0, 5), "\0\0\0\0\0");
        assert_eq!(extract_line(&buffer, 0, 1, 22), "his is a span of text\0");

        buffer.clear(true, true, true);
        let (left, right) = line.split_at_space(line.char_len() - 1);
        {
            println!("left={:?}, right={:?}", left, right);
            let mut printer = colored(&mut buffer);
            left.print(&mut printer, 0, 0);
            right.print(&mut printer, 0, 1);
        }
        assert_eq!(extract_line(&buffer, 0, 0, 22), "This is a span of tex\0");
        assert_eq!(extract_line(&buffer, 0, 1, 2), "\0\0");
    }

    #[test]
    fn span_split_at_char() {
        let text = "This is a span of text";
        let span = ColoredSpan::new(Some("color"), text);

        let (left, right) = span.split_at_char(9);
        assert_eq!(left.txt, "This is a");
        assert_eq!(right.txt, " span of text");
        assert_eq!(left.color, Some("color"));
        assert_eq!(right.color, Some("color"));

        let (left, right) = span.split_at_char(0);
        assert_eq!(left.txt, "");
        assert_eq!(right.txt, "This is a span of text");
        assert_eq!(left.color, Some("color"));
        assert_eq!(right.color, Some("color"));

        let (left, right) = span.split_at_char(span.char_len() - 1);
        assert_eq!(left.txt, "This is a span of tex");
        assert_eq!(right.txt, "t");
        assert_eq!(left.color, Some("color"));
        assert_eq!(right.color, Some("color"));
    }

    #[test]
    fn line_hyphenate_at_char() {
        let text = "This is a #[00F]span#[] of text";
        let mut colors = Vec::new();
        let line = parse_colored_line(text, &mut colors);
        let mut buffer = Buffer::new(50, 50);

        let (left, right) = line.hyphenate_at_char(12);
        {
            let mut printer = colored(&mut buffer);
            left.print(&mut printer, 0, 0);
            right.print(&mut printer, 0, 1);
        }
        assert_eq!(extract_line(&buffer, 0, 0, 14), "This is a sp-\0");
        assert_eq!(extract_line(&buffer, 0, 1, 11), "an of text\0");

        buffer.clear(true, true, true);
        let (left, right) = line.hyphenate_at_char(0);
        {
            println!("left={:?}, right={:?}", left, right);
            let mut printer = colored(&mut buffer);
            left.print(&mut printer, 0, 0);
            right.print(&mut printer, 0, 1);
        }
        assert_eq!(extract_line(&buffer, 0, 0, 5), "\0\0\0\0\0");
        assert_eq!(extract_line(&buffer, 0, 1, 23), "This is a span of text\0");

        buffer.clear(true, true, true);
        let (left, right) = line.hyphenate_at_char(line.char_len() - 1);
        {
            println!("left={:?}, right={:?}", left, right);
            let mut printer = colored(&mut buffer);
            left.print(&mut printer, 0, 0);
            right.print(&mut printer, 0, 1);
        }
        assert_eq!(extract_line(&buffer, 0, 0, 23), "This is a span of tex-\0");
        assert_eq!(extract_line(&buffer, 0, 1, 2), "t\0");
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

        assert_eq!(printer.wrap(0, 0, "taco casa"), (10, 1));
        assert_eq!(extract_line(&buffer, 0, 0, 10), "taco casa\0");
    }

    #[test]
    fn wrap_multi_plain() {
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
        assert_eq!(extract_line(&buffer, 0, 4, 21), "stonemasons' monume-\0");
        assert_eq!(
            extract_line(&buffer, 0, 5, 21),
            "nts\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"
        );
        assert_eq!(r, (20, 5));
    }
}
