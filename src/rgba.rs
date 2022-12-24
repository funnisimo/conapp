use crate::console;
use std::ops;

pub type RGB = (u8, u8, u8);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct RGBA(pub u8, pub u8, pub u8, pub u8);

impl RGBA {
    pub const fn new() -> Self {
        RGBA(0, 0, 0, 0)
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        RGBA(r, g, b, 255)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        RGBA(r, g, b, a)
    }

    pub fn r(&self) -> u8 {
        self.0
    }
    pub fn g(&self) -> u8 {
        self.1
    }
    pub fn b(&self) -> u8 {
        self.2
    }
    pub fn a(&self) -> u8 {
        self.3
    }

    pub fn as_f32(&self) -> (f32, f32, f32, f32) {
        (
            self.0 as f32 / 255.0,
            self.1 as f32 / 255.0,
            self.2 as f32 / 255.0,
            self.3 as f32 / 255.0,
        )
    }
}

impl From<RGB> for RGBA {
    fn from(d: RGB) -> Self {
        RGBA::rgb(d.0, d.1, d.2)
    }
}

impl From<(u8, u8, u8, u8)> for RGBA {
    fn from(d: (u8, u8, u8, u8)) -> Self {
        RGBA::rgba(d.0, d.1, d.2, d.3)
    }
}

impl From<(f32, f32, f32)> for RGBA {
    fn from(d: (f32, f32, f32)) -> Self {
        let r = (d.0 * 255.0).floor().clamp(0.0, 255.0) as u8;
        let g = (d.1 * 255.0).floor().clamp(0.0, 255.0) as u8;
        let b = (d.2 * 255.0).floor().clamp(0.0, 255.0) as u8;
        RGBA::rgb(r, g, b)
    }
}

impl From<(f32, f32, f32, f32)> for RGBA {
    fn from(d: (f32, f32, f32, f32)) -> Self {
        let r = (d.0 * 255.0).floor().clamp(0.0, 255.0) as u8;
        let g = (d.1 * 255.0).floor().clamp(0.0, 255.0) as u8;
        let b = (d.2 * 255.0).floor().clamp(0.0, 255.0) as u8;
        let a = (d.3 * 255.0).floor().clamp(0.0, 255.0) as u8;
        RGBA::rgba(r, g, b, a)
    }
}

impl Into<(u8, u8, u8, u8)> for RGBA {
    fn into(self) -> (u8, u8, u8, u8) {
        (self.0, self.1, self.2, self.3)
    }
}

impl Into<(f32, f32, f32, f32)> for RGBA {
    fn into(self) -> (f32, f32, f32, f32) {
        self.as_f32()
    }
}

// impl PartialEq for RGBA {
//     fn eq(&self, other: &Self) -> bool {
//         self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
//     }
// }

impl ops::Mul<f32> for RGBA {
    type Output = RGBA;

    fn mul(self, rhs: f32) -> Self::Output {
        let r = (self.0 as f32 * rhs).round().clamp(0.0, 255.0) as u8;
        let g = (self.1 as f32 * rhs).round().clamp(0.0, 255.0) as u8;
        let b = (self.2 as f32 * rhs).round().clamp(0.0, 255.0) as u8;
        RGBA::rgba(r, g, b, self.3)
    }
}

impl ops::Mul<f32> for &RGBA {
    type Output = RGBA;

    fn mul(self, rhs: f32) -> Self::Output {
        let r = (self.0 as f32 * rhs).round().clamp(0.0, 255.0) as u8;
        let g = (self.1 as f32 * rhs).round().clamp(0.0, 255.0) as u8;
        let b = (self.2 as f32 * rhs).round().clamp(0.0, 255.0) as u8;
        RGBA::rgba(r, g, b, self.3)
    }
}

impl ops::Add<RGBA> for RGBA {
    type Output = RGBA;

    fn add(self, rhs: RGBA) -> Self::Output {
        let r = self.0.saturating_add(rhs.0);
        let g = self.1.saturating_add(rhs.1);
        let b = self.2.saturating_add(rhs.2);
        let a = self.3.saturating_add(rhs.3); // hmmm?
        RGBA::rgba(r, g, b, a)
    }
}

impl ops::Add<RGBA> for &RGBA {
    type Output = RGBA;

    fn add(self, rhs: RGBA) -> Self::Output {
        let r = self.0.saturating_add(rhs.0);
        let g = self.1.saturating_add(rhs.1);
        let b = self.2.saturating_add(rhs.2);
        let a = self.3.saturating_add(rhs.3); // hmmm?
        RGBA::rgba(r, g, b, a)
    }
}

pub fn color_blend(c1: RGBA, c2: RGBA, pct: f32) -> RGBA {
    let alpha = pct * c2.3 as f32 / 255.0;
    RGBA::rgba(
        ((1.0 - alpha) * f32::from(c1.0) + alpha * f32::from(c2.0)) as u8,
        ((1.0 - alpha) * f32::from(c1.1) + alpha * f32::from(c2.1)) as u8,
        ((1.0 - alpha) * f32::from(c1.2) + alpha * f32::from(c2.2)) as u8,
        255,
    )
}

pub fn color_lerp(c1: RGBA, c2: RGBA, pct: f32) -> RGBA {
    RGBA::rgba(
        ((1.0 - pct) * f32::from(c1.0) + pct * f32::from(c2.0)) as u8,
        ((1.0 - pct) * f32::from(c1.1) + pct * f32::from(c2.1)) as u8,
        ((1.0 - pct) * f32::from(c1.2) + pct * f32::from(c2.2)) as u8,
        ((1.0 - pct) * f32::from(c1.3) + pct * f32::from(c2.3)) as u8,
    )
}

pub fn color_scale(c: RGBA, coef: f32) -> RGBA {
    RGBA::rgba(
        (f32::from(c.0) * coef).min(255.0) as u8,
        (f32::from(c.1) * coef).min(255.0) as u8,
        (f32::from(c.2) * coef).min(255.0) as u8,
        c.3,
    )
}

pub fn color_mul(c1: RGBA, c2: RGBA) -> RGBA {
    RGBA::rgba(
        (f32::from(c1.0) * f32::from(c2.0) / 255.0) as u8,
        (f32::from(c1.1) * f32::from(c2.1) / 255.0) as u8,
        (f32::from(c1.2) * f32::from(c2.2) / 255.0) as u8,
        255,
    )
}

pub fn color_add(c1: RGBA, c2: RGBA) -> RGBA {
    RGBA::rgba(
        (0.5 * f32::from(c1.0) + 0.5 * f32::from(c2.0)) as u8,
        (0.5 * f32::from(c1.1) + 0.5 * f32::from(c2.1)) as u8,
        (0.5 * f32::from(c1.2) + 0.5 * f32::from(c2.2)) as u8,
        (0.5 * f32::from(c1.3) + 0.5 * f32::from(c2.3)) as u8,
    )
}

pub fn color_dist(c1: RGBA, c2: RGBA) -> i32 {
    let dr = i32::from(c1.0) - i32::from(c2.0);
    let dg = i32::from(c1.1) - i32::from(c2.1);
    let db = i32::from(c1.2) - i32::from(c2.2);
    dr * dr + dg * dg + db * db
}

#[derive(Debug, Copy, Clone)]
pub enum ColorParseErr {
    NonHexDigit,
    NonAsciiDigit,
    WrongHexLen,
    WrongRgbLen,
}

pub fn parse_color_hex(text: &str) -> Result<RGBA, ColorParseErr> {
    let base = match text.starts_with("#") {
        false => text,
        true => &text[1..],
    };

    if !base.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(ColorParseErr::NonHexDigit);
    }

    let digits: Vec<u32> = base
        .chars()
        .map(|ch| ch.to_digit(16).unwrap_or(0))
        .collect();

    let (r, g, b, a) = match digits.len() {
        3 => (
            digits[0] as f32 / 15.0,
            digits[1] as f32 / 15.0,
            digits[2] as f32 / 15.0,
            1.0,
        ),
        4 => (
            digits[0] as f32 / 15.0,
            digits[1] as f32 / 15.0,
            digits[2] as f32 / 15.0,
            digits[3] as f32 / 15.0,
        ),
        6 => (
            (digits[0] as f32 * 16.0 + digits[1] as f32) / 255.0,
            (digits[2] as f32 * 16.0 + digits[3] as f32) / 255.0,
            (digits[4] as f32 * 16.0 + digits[5] as f32) / 255.0,
            1.0,
        ),
        8 => (
            (digits[0] as f32 * 16.0 + digits[1] as f32) / 255.0,
            (digits[2] as f32 * 16.0 + digits[3] as f32) / 255.0,
            (digits[4] as f32 * 16.0 + digits[5] as f32) / 255.0,
            (digits[6] as f32 * 16.0 + digits[7] as f32) / 255.0,
        ),
        _ => {
            return Err(ColorParseErr::WrongHexLen);
        }
    };

    Ok((r, g, b, a).into())
}

pub fn parse_color_rgb(text: &str) -> Result<RGBA, ColorParseErr> {
    let start = match text.chars().position(|ch| ch == '(') {
        None => text,
        Some(idx) => &text[idx + 1..],
    };

    let body = match start.chars().position(|ch| ch == ')') {
        None => start,
        Some(idx) => &start[..idx],
    };

    // println!("color guts = {}", &text[start..end + start]);

    let num_parts = body.split(",").map(|p| p.trim()).collect::<Vec<&str>>();

    if num_parts.len() != 3 && num_parts.len() != 4 {
        return Err(ColorParseErr::WrongRgbLen);
    }

    let mut nums: Vec<u8> = Vec::new();
    for part in num_parts {
        if !part.chars().all(|ch| ch.is_ascii_digit()) {
            return Err(ColorParseErr::NonAsciiDigit);
        }
        match part.parse::<u8>() {
            Err(_) => return Err(ColorParseErr::NonAsciiDigit),
            Ok(v) => nums.push(v),
        }
    }

    match nums.len() {
        3 => return Ok((nums[0], nums[1], nums[2], 255).into()),
        4 => return Ok((nums[0], nums[1], nums[2], nums[3]).into()),
        _ => {
            return Err(ColorParseErr::WrongRgbLen);
        }
    }
}

pub fn parse_color(name: &str) -> Result<RGBA, ColorParseErr> {
    let name = name.trim().to_lowercase();
    if name.starts_with("#") {
        // skip down...
    } else if name.starts_with("(")
        || name.starts_with("rgb(")
        || name.starts_with("rgba(")
        || name.contains(",")
    {
        return parse_color_rgb(&name);
    }
    parse_color_hex(&name)
}

pub fn to_rgba(name: &str) -> Option<RGBA> {
    match parse_color(name) {
        Err(e) => {
            console(format!("{:?}", e));
            None
        }
        Ok(c) => Some(c),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const WHITE: RGBA = RGBA::rgb(255, 255, 255);
    const RED: RGBA = RGBA::rgb(255, 0, 0);
    const GREEN: RGBA = RGBA::rgb(0, 255, 0);
    const BLUE: RGBA = RGBA::rgb(0, 0, 255);
    const _BLACK: RGBA = RGBA::rgb(0, 0, 0);

    #[test]
    fn parse_hex() {
        assert_eq!(parse_color_hex("#fff").unwrap(), WHITE);
        assert_eq!(parse_color_hex("#ffff").unwrap(), WHITE);
        assert_eq!(parse_color_hex("#ffffff").unwrap(), WHITE);
        assert_eq!(parse_color_hex("#ffffffff").unwrap(), WHITE);

        assert_eq!(parse_color_hex("#f00").unwrap(), RED);
        assert_eq!(parse_color_hex("#0f0f").unwrap(), GREEN);
        assert_eq!(parse_color_hex("#0000ff").unwrap(), BLUE);
        assert_eq!(
            parse_color_hex("#80808080").unwrap(),
            RGBA::rgba(128, 128, 128, 128)
        );

        assert_eq!(parse_color_hex("F00").unwrap(), RED);
        assert_eq!(parse_color_hex("0F0F").unwrap(), GREEN);
        assert_eq!(parse_color_hex("0000FF").unwrap(), BLUE);
        assert_eq!(
            parse_color_hex("80808080").unwrap(),
            RGBA::rgba(128, 128, 128, 128)
        );

        assert!(parse_color_hex("white").is_err());
        assert!(parse_color_hex("12,34,56").is_err());
    }

    #[test]
    fn parse_rgb() {
        assert_eq!(parse_color_rgb("0,0,0").unwrap(), RGBA::rgba(0, 0, 0, 255));

        assert_eq!(
            parse_color_rgb("rgb(10,20,30)").unwrap(),
            RGBA::rgba(10, 20, 30, 255)
        );

        assert_eq!(
            parse_color_rgb("(255,150,200,25)").unwrap(),
            RGBA::rgba(255, 150, 200, 25)
        );

        assert_eq!(
            parse_color_rgb("rgba(10,20,30)").unwrap(),
            RGBA::rgba(10, 20, 30, 255)
        );

        assert!(parse_color_rgb("FFF").is_err());
        assert!(parse_color_rgb("white").is_err());
    }

    #[test]
    fn parse_test() {
        assert_eq!(parse_color("0,0,0").unwrap(), RGBA::rgba(0, 0, 0, 255));

        assert_eq!(
            parse_color("rgb(10,20,30)").unwrap(),
            RGBA::rgba(10, 20, 30, 255)
        );

        assert_eq!(
            parse_color("(255,150,200,25)").unwrap(),
            RGBA::rgba(255, 150, 200, 25)
        );

        assert_eq!(
            parse_color("rgba(10,20,30)").unwrap(),
            RGBA::rgba(10, 20, 30, 255)
        );

        assert_eq!(parse_color("#f00").unwrap(), RED);
        assert_eq!(parse_color("#0f0f").unwrap(), GREEN);
        assert_eq!(parse_color("#0000ff").unwrap(), BLUE);
        assert_eq!(
            parse_color("#80808080").unwrap(),
            RGBA::rgba(128, 128, 128, 128)
        );

        assert_eq!(parse_color("F00").unwrap(), RED);
        assert_eq!(parse_color("0F0F").unwrap(), GREEN);
        assert_eq!(parse_color("0000FF").unwrap(), BLUE);
        assert_eq!(
            parse_color("80808080").unwrap(),
            RGBA::rgba(128, 128, 128, 128)
        );

        assert!(parse_color("white").is_err());
        assert!(parse_color("WHITE").is_err());
    }
}
