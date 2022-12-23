#![warn(clippy::float_cmp)]
use crate::console;
use crate::file::FileLoader;
use crate::rgba::RGBA;
use crate::Glyph;
use std::cell::RefCell;
use std::rc::Rc;

// sub-pixel resolution kit
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SubCellFlag {
    NONE = 0,
    NW,
    NE,
    N,
    SE,
    DIAG,
    E,
    SW,
}

pub fn to_subcell_glyph(subcell: SubCellFlag) -> Glyph {
    subcell as Glyph
}

// const CHAR_SUBP_NW: u32 = 226;
// const CHAR_SUBP_NE: u32 = 227;
// const CHAR_SUBP_N: u32 = 228;
// const CHAR_SUBP_SE: u32 = 229;
// const CHAR_SUBP_DIAG: u32 = 230;
// const CHAR_SUBP_E: u32 = 231;
// const CHAR_SUBP_SW: u32 = 232;

/// An easy way to load PNG images and blit them on the console
pub struct Image {
    file_loader: FileLoader,
    pub(crate) img: Option<image::RgbaImage>,
}

impl Image {
    /// Create an image and load a PNG file.
    /// On the web platform, image loading is asynchronous.
    /// Using blit methods before the image is loaded has no impact on the console.
    pub(crate) fn new(file_path: &str) -> Rc<RefCell<Self>> {
        let mut file_loader = FileLoader::new();
        file_loader
            .load_file(file_path)
            .expect("Image file load failed.");

        Rc::new(RefCell::new(Self {
            file_loader,
            img: None,
        }))
    }
    /// Returns the image's width in pixels or 0 if the image has not yet been loaded
    pub fn width(&self) -> u32 {
        if let Some(ref img) = self.img {
            return img.width();
        }
        0
    }
    /// Returns the image's height in pixels or 0 if the image has not yet been loaded
    pub fn height(&self) -> u32 {
        if let Some(ref img) = self.img {
            return img.height();
        }
        0
    }
    /// Create an empty image.
    pub fn new_empty(width: u32, height: u32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            file_loader: FileLoader::new(),
            img: Some(image::RgbaImage::new(width, height)),
        }))
    }
    /// get the color of a specific pixel inside the image
    pub fn pixel(&self, x: u32, y: u32) -> Option<RGBA> {
        if let Some(ref img) = self.img {
            let p = img.get_pixel(x, y);
            return Some(RGBA::rgba(p[0], p[1], p[2], p[3]));
        }
        None
    }
    /// sets the color of a specific pixel inside the image
    pub fn put_pixel(&mut self, x: u32, y: u32, color: RGBA) {
        if let Some(ref mut img) = self.img {
            img.put_pixel(x, y, image::Rgba([color.0, color.1, color.2, color.3]));
        }
    }
    /// Check if the image has been loaded.
    /// Since there's no background thread doing the work for you, you have to call some method on image for it to actually load.
    /// Use either [`Image::try_load`], [`Image::get_size`], [`Image::blit`] or [`Image::blit_ex`] to run the loading code.
    pub(crate) fn load_async(&mut self) -> bool {
        if self.img.is_some() {
            return true;
        }
        if self.file_loader.check_file_ready(0) {
            console("img loaded");
            let buf = self.file_loader.get_file_content(0);
            self.intialize_image(&buf);
            return true;
        }
        false
    }

    pub fn is_loaded(&self) -> bool {
        self.img.is_some()
    }

    fn intialize_image(&mut self, buf: &[u8]) {
        self.img = Some(image::load_from_memory(buf).unwrap().to_rgba8());
    }

    /// If the image has already been loaded, return its size, else return None
    pub fn get_size(&self) -> Option<(u32, u32)> {
        if self.is_loaded() {
            if let Some(ref img) = self.img {
                return Some((img.width(), img.height()));
            }
        }
        None
    }

    // /// blit an image on the console, using the subcell characters to achieve twice the normal resolution.
    // /// This uses the CHAR_SUBCELL_* ascii codes (from 226 to 232):
    // ///
    // /// ![subcell_chars](https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/subcell/subcell.png)
    // ///
    // /// Comparison before/after subcell in the chronicles of Doryen :
    // ///
    // /// ![subcell_comp](https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/subcell/subcell_comp.png)
    // ///
    // /// Pyromancer! screenshot, making full usage of subcell resolution:
    // ///
    // /// ![subcell_pyro](https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/subcell/subcell_pyro.png)
    // pub fn blit_2x(
    //     &mut self,
    //     con: &mut Buffer,
    //     dx: i32,
    //     dy: i32,
    //     sx: i32,
    //     sy: i32,
    //     w: Option<i32>,
    //     h: Option<i32>,
    //     transparent: Option<RGBA>,
    // ) {
    //     if !self.try_load() {
    //         return;
    //     }
    //     if let Some(ref img) = self.img {
    //         Image::blit_2x_image(img, con, dx, dy, sx, sy, w, h, transparent);
    //     }
    // }
    // /// blit an image on a console. See [`Image::blit_2x`]
    //  fn blit_2x_image(
    //     img: &image::RgbaImage,
    //     con: &mut Buffer,
    //     dx: i32,
    //     dy: i32,
    //     sx: i32,
    //     sy: i32,
    //     w: Option<i32>,
    //     h: Option<i32>,
    //     transparent: Option<RGBA>,
    // ) {
    //     let mut grid: [RGBA; 4] = [
    //         (0, 0, 0, 0).into(),
    //         (0, 0, 0, 0).into(),
    //         (0, 0, 0, 0).into(),
    //         (0, 0, 0, 0).into(),
    //     ];
    //     let mut back: RGBA = (0, 0, 0, 0).into();
    //     let mut front: Option<RGBA> = None;
    //     let mut ascii: i32 = ' ' as i32;
    //     let width = img.width() as i32;
    //     let height = img.height() as i32;
    //     let con_width = con.get_width() as i32;
    //     let con_height = con.get_height() as i32;
    //     let mut blit_w = w.unwrap_or(width);
    //     let mut blit_h = h.unwrap_or(height);
    //     let minx = sx.max(0);
    //     let miny = sy.max(0);
    //     blit_w = blit_w.min(width - minx);
    //     blit_h = blit_h.min(height - miny);
    //     let mut maxx = if dx + blit_w / 2 <= con_width {
    //         blit_w
    //     } else {
    //         (con_width - dx) * 2
    //     };
    //     let mut maxy = if dy + blit_h / 2 <= con_height {
    //         blit_h
    //     } else {
    //         (con_height - dy) * 2
    //     };
    //     maxx += minx;
    //     maxy += miny;
    //     let mut cx = minx;
    //     while cx < maxx {
    //         let mut cy = miny;
    //         while cy < maxy {
    //             // get the 2x2 super pixel colors from the image
    //             let conx = dx + (cx - minx) / 2;
    //             let cony = dy + (cy - miny) / 2;
    //             let console_back = con.get_back(conx, cony).unwrap().clone();
    //             let pixel = img.get_pixel(cx as u32, cy as u32);
    //             grid[0] = RGBA::rgba(pixel[0], pixel[1], pixel[2], pixel[3]);
    //             if let Some(ref t) = transparent {
    //                 if grid[0] == *t {
    //                     grid[0] = console_back;
    //                 }
    //             }
    //             if cx < maxx - 1 {
    //                 let pixel = img.get_pixel(cx as u32 + 1, cy as u32);
    //                 grid[1] = RGBA::rgba(pixel[0], pixel[1], pixel[2], pixel[3]);
    //                 if let Some(ref t) = transparent {
    //                     if grid[1] == *t {
    //                         grid[1] = console_back;
    //                     }
    //                 }
    //             } else {
    //                 grid[1] = console_back;
    //             }
    //             if cy < maxy - 1 {
    //                 let pixel = img.get_pixel(cx as u32, cy as u32 + 1);
    //                 grid[2] = RGBA::rgba(pixel[0], pixel[1], pixel[2], pixel[3]);
    //                 if let Some(ref t) = transparent {
    //                     if grid[2] == *t {
    //                         grid[2] = console_back;
    //                     }
    //                 }
    //             } else {
    //                 grid[2] = console_back;
    //             }
    //             if cx < maxx - 1 && cy < maxy - 1 {
    //                 let pixel = img.get_pixel(cx as u32 + 1, cy as u32 + 1);
    //                 grid[3] = RGBA::rgba(pixel[0], pixel[1], pixel[2], pixel[3]);
    //                 if let Some(ref t) = transparent {
    //                     if grid[3] == *t {
    //                         grid[3] = console_back;
    //                     }
    //                 }
    //             } else {
    //                 grid[3] = console_back;
    //             }
    //             // analyse color, posterize, get pattern
    //             compute_pattern(&grid, &mut back, &mut front, &mut ascii);
    //             if let Some(front) = front {
    //                 if ascii >= 0 {
    //                     con.back(conx, cony, back);
    //                     con.fore(conx, cony, front);
    //                     con.glyph(conx, cony, ascii as u32);
    //                 } else {
    //                     con.back(conx, cony, front);
    //                     con.fore(conx, cony, back);
    //                     con.glyph(conx, cony, (-ascii) as u32);
    //                 }
    //             } else {
    //                 // single color
    //                 con.back(conx, cony, back);
    //                 con.glyph(conx, cony, ascii as u32);
    //             }
    //             cy += 2;
    //         }
    //         cx += 2;
    //     }
    // }
}

// const FLAG_TO_ASCII: [i32; 8] = [
//     0,
//     CHAR_SUBP_NE as i32,
//     CHAR_SUBP_SW as i32,
//     CHAR_SUBP_DIAG as i32,
//     CHAR_SUBP_SE as i32,
//     CHAR_SUBP_E as i32,
//     -(CHAR_SUBP_N as i32),
//     -(CHAR_SUBP_NW as i32),
// ];

// fn compute_pattern(
//     desired: &[RGBA; 4],
//     back: &mut RGBA,
//     front: &mut Option<RGBA>,
//     ascii: &mut i32,
// ) {
//     // adapted from Jeff Lait's code posted on r.g.r.d
//     let mut flag = 0;
//     /*
//         pixels have following flag values :
//             X 1
//             2 4
//         flag indicates which pixels uses foreground color (top left pixel always uses foreground color except if all pixels have the same color)
//     */
//     let mut weight: [f32; 2] = [0.0, 0.0];
//     // First colour trivial.
//     *back = desired[0];

//     // Ignore all duplicates...
//     let mut i = 1;
//     while i < 4 {
//         if desired[i].0 != back.0 || desired[i].1 != back.1 || desired[i].2 != back.2 {
//             break;
//         }
//         i += 1;
//     }

//     // All the same.
//     if i == 4 {
//         *front = None;
//         *ascii = ' ' as i32;
//         return;
//     }
//     weight[0] = i as f32;

//     // Found a second colour...
//     let mut tmp_front = desired[i];
//     weight[1] = 1.0;
//     flag |= 1 << (i - 1);
//     // remaining colours
//     i += 1;
//     while i < 4 {
//         if desired[i].0 == back.0 && desired[i].1 == back.1 && desired[i].2 == back.2 {
//             weight[0] += 1.0;
//         } else if desired[i].0 == tmp_front.0
//             && desired[i].1 == tmp_front.1
//             && desired[i].2 == tmp_front.2
//         {
//             flag |= 1 << (i - 1);
//             weight[1] += 1.0;
//         } else {
//             // Bah, too many colours,
//             // merge the two nearest
//             let dist0i = color_dist(desired[i], *back);
//             let dist1i = color_dist(desired[i], tmp_front);
//             let dist01 = color_dist(*back, tmp_front);
//             if dist0i < dist1i {
//                 if dist0i <= dist01 {
//                     // merge 0 and i
//                     *back = color_blend(desired[i], *back, weight[0] / (1.0 + weight[0]));
//                     weight[0] += 1.0;
//                 } else {
//                     // merge 0 and 1
//                     *back = color_blend(*back, tmp_front, weight[1] / (weight[0] + weight[1]));
//                     weight[0] += 1.0;
//                     tmp_front = desired[i];
//                     flag = 1 << (i - 1);
//                 }
//             } else if dist1i <= dist01 {
//                 // merge 1 and i
//                 tmp_front = color_blend(desired[i], tmp_front, weight[1] / (1.0 + weight[1]));
//                 weight[1] += 1.0;
//                 flag |= 1 << (i - 1);
//             } else {
//                 // merge 0 and 1
//                 *back = color_blend(*back, tmp_front, weight[1] / (weight[0] + weight[1]));
//                 weight[0] += 1.0;
//                 tmp_front = desired[i];
//                 flag = 1 << (i - 1);
//             }
//         }
//         i += 1;
//     }
//     *front = Some(tmp_front);
//     *ascii = FLAG_TO_ASCII[flag as usize];
// }
