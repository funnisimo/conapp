use std::ops;

pub type RGB = (u8, u8, u8);

#[derive(Copy, Clone, Debug, Default)]
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

impl PartialEq for RGBA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
    }
}

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
        let a = self.3.saturating_add(rhs.3);
        RGBA::rgba(r, g, b, a)
    }
}

impl ops::Add<RGBA> for &RGBA {
    type Output = RGBA;

    fn add(self, rhs: RGBA) -> Self::Output {
        let r = self.0.saturating_add(rhs.0);
        let g = self.1.saturating_add(rhs.1);
        let b = self.2.saturating_add(rhs.2);
        let a = self.3.saturating_add(rhs.3);
        RGBA::rgba(r, g, b, a)
    }
}

pub(super) fn color_blend(c1: RGBA, c2: RGBA, alpha: f32) -> RGBA {
    let alpha = alpha * c2.3 as f32 / 255.0;
    RGBA::rgba(
        ((1.0 - alpha) * f32::from(c1.0) + alpha * f32::from(c2.0)) as u8,
        ((1.0 - alpha) * f32::from(c1.1) + alpha * f32::from(c2.1)) as u8,
        ((1.0 - alpha) * f32::from(c1.2) + alpha * f32::from(c2.2)) as u8,
        255,
    )
}

/*
pub(super) fn color_scale(c: RGBA, coef: f32) -> RGBA {
    RGBA::rgba(
        (f32::from(c.0) * coef).min(255.0) as u8,
        (f32::from(c.1) * coef).min(255.0) as u8,
        (f32::from(c.2) * coef).min(255.0) as u8,
        c.3,
    )
}

pub(super) fn color_mul(c1: RGBA, c2: RGBA) -> RGBA {
    RGBA::rgba(
        (f32::from(c1.0) * f32::from(c2.0) / 255.0) as u8,
        (f32::from(c1.1) * f32::from(c2.1) / 255.0) as u8,
        (f32::from(c1.2) * f32::from(c2.2) / 255.0) as u8,
        255,
    )
}

pub(super) fn color_add(c1: RGBA, c2: RGBA) -> RGBA {
    RGBA::rgba(
        (0.5 * f32::from(c1.0) + 0.5 * f32::from(c2.0)) as u8,
        (0.5 * f32::from(c1.1) + 0.5 * f32::from(c2.1)) as u8,
        (0.5 * f32::from(c1.2) + 0.5 * f32::from(c2.2)) as u8,
        (0.5 * f32::from(c1.3) + 0.5 * f32::from(c2.3)) as u8,
    )
}
*/

pub fn color_dist(c1: RGBA, c2: RGBA) -> i32 {
    let dr = i32::from(c1.0) - i32::from(c2.0);
    let dg = i32::from(c1.1) - i32::from(c2.1);
    let db = i32::from(c1.2) - i32::from(c2.2);
    dr * dr + dg * dg + db * db
}

pub fn parse_color_hex(text: &str) -> Result<RGBA, String> {
    if !text.starts_with("#") {
        return Err(format!("Hex color does not start with hash(#) - {}", text));
    }

    let digits: Vec<u32> = text
        .chars()
        .skip(1)
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
            return Err(format!(
                "Hex color must be in one of these formats - #abc, #abcd, #aabbcc, or #aabbccdd : {}",
                text
            ));
        }
    };

    Ok((r, g, b, a).into())
}

pub fn parse_color_rgb(text: &str) -> Result<RGBA, String> {
    let start = match text.chars().position(|ch| ch == '(') {
        None => {
            return Err(format!(
                "Color must start with either 'rgb(' or '(' - found: {}",
                text
            ))
        }
        Some(idx) => idx + 1,
    };

    let end = match text.chars().skip(start).position(|ch| ch == ')') {
        None => return Err(format!("Color must have closing ')' - found: {}", text)),
        Some(idx) => idx,
    };

    // println!("color guts = {}", &text[start..end + start]);

    let num_parts = text[start..end + start]
        .split(",")
        .map(|p| p.trim())
        .collect::<Vec<&str>>();

    if num_parts.len() != 3 && num_parts.len() != 4 {
        return Err(format!("Expected 3 or 4 color values (0-255) - {}", text));
    }

    let mut nums: Vec<u8> = Vec::new();
    for part in num_parts {
        match part.parse::<u8>() {
            Err(e) => {
                return Err(format!(
                    "Failed to convert color component - {} : {} : {}",
                    part,
                    text,
                    e.to_string()
                ))
            }
            Ok(v) => nums.push(v),
        }
    }

    match nums.len() {
        3 => return Ok((nums[0], nums[1], nums[2], 255).into()),
        4 => return Ok((nums[0], nums[1], nums[2], nums[3]).into()),
        _ => {
            return Err(format!(
                "Did not get expected number of color components (3 or 4) - {}",
                text
            ));
        }
    }
}

pub fn parse_color(name: &str) -> Option<RGBA> {
    let name = name.trim().to_lowercase();
    if name.starts_with("#") {
        match parse_color_hex(&name) {
            Err(e) => {
                crate::log(&e);
                return None;
            }
            Ok(rgba) => return Some(rgba),
        }
    } else if name.starts_with("(") || name.starts_with("rgb(") || name.starts_with("rgba(") {
        match parse_color_rgb(&name) {
            Err(e) => {
                crate::log(&e);
                return None;
            }
            Ok(rgba) => return Some(rgba),
        }
    }
    None
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
    }

    #[test]
    fn parse_rgb() {
        assert!(parse_color_rgb("0,0,0").is_err());

        assert_eq!(
            parse_color_rgb("rgb(10,20,30)").unwrap(),
            RGBA::rgba(10, 20, 30, 255)
        );

        assert_eq!(
            parse_color_rgb("(255,150,200,25)").unwrap(),
            RGBA::rgba(255, 150, 200, 25)
        );
    }
}
