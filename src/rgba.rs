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

pub(super) fn color_dist(c1: RGBA, c2: RGBA) -> i32 {
    let dr = i32::from(c1.0) - i32::from(c2.0);
    let dg = i32::from(c1.1) - i32::from(c2.1);
    let db = i32::from(c1.2) - i32::from(c2.2);
    dr * dr + dg * dg + db * db
}
