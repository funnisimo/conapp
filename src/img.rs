#![warn(clippy::float_cmp)]
use crate::rgba::RGBA;
// use std::cell::RefCell;
// use std::rc::Rc;

/// An easy way to load PNG images and blit them on the console
pub struct Image {
    // file_loader: FileLoader,
    pub(crate) id: usize,
    pub(crate) img: Option<image::RgbaImage>,
}

impl Image {
    /// Create an empty image.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            id: 0,
            // file_loader: FileLoader::new(),
            img: Some(image::RgbaImage::new(width, height)),
        }
    }

    /// Create an image that will be loaded.
    /// On the web platform, image loading is asynchronous.
    /// Using blit methods before the image is loaded has no impact on the console.
    pub(crate) fn new_async(id: usize) -> Self {
        Self {
            id,
            // file_loader,
            img: None,
        }
    }

    /// Returns the image's width in pixels or 0 if the image has not yet been loaded
    pub fn width(&self) -> u32 {
        match self.img {
            None => 0,
            Some(ref img) => img.width(),
        }
    }
    /// Returns the image's height in pixels or 0 if the image has not yet been loaded
    pub fn height(&self) -> u32 {
        match self.img {
            None => 0,
            Some(ref img) => img.height(),
        }
    }

    pub fn img(&self) -> Option<&image::RgbaImage> {
        self.img.as_ref()
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

    // /// Check if the image has been loaded.
    // /// Since there's no background thread doing the work for you, you have to call some method on image for it to actually load.
    // /// Use either [`Image::try_load`], [`Image::get_size`], [`Image::blit`] or [`Image::blit_ex`] to run the loading code.
    // pub(crate) fn load_async(&mut self) -> bool {
    //     if self.img.is_some() {
    //         return true;
    //     }
    //     if self.file_loader.check_file_ready(0) {
    //         console("img loaded");
    //         let buf = self.file_loader.get_file_content(0);
    //         self.intialize_image(&buf);
    //         return true;
    //     }
    //     false
    // }

    pub fn is_loaded(&self) -> bool {
        self.img.is_some()
    }

    pub(crate) fn intialize_image(&mut self, buf: &[u8]) {
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
}
