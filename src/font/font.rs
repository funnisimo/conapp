use super::*;
use image::{ImageBuffer, Rgba};

pub struct Font {
    img_width: u32,
    img_height: u32,
    char_width: u32,
    char_height: u32,
    len: u32,

    path: String,
    loaded: bool,
    // path: Option<String>,
    loader: FontLoader,
}

impl Font {
    pub fn new(path: &str) -> Self {
        let mut loader = FontLoader::new();
        crate::App::print(format!("Loading font - {}", path));
        loader.load_font(path);

        Font {
            img_width: 0,
            img_height: 0,
            char_width: 0,
            char_height: 0,
            len: 0,

            path: path.to_owned(),
            loaded: false,
            loader,
        }
    }

    pub fn img_width(&self) -> u32 {
        self.img_width
    }
    pub fn img_height(&self) -> u32 {
        self.img_height
    }
    pub fn char_width(&self) -> u32 {
        self.char_width
    }
    pub fn char_height(&self) -> u32 {
        self.char_height
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn loaded(&self) -> bool {
        self.loaded
    }

    pub fn take_img(&self) -> &ImageBuffer<Rgba<u8>, Vec<u8>> {
        self.loader.img.as_ref().unwrap()
    }

    pub fn load_async(&mut self) -> bool {
        if self.loaded {
            return true;
        }

        if !self.loader.load_font_async() {
            crate::App::print(format!("-loading font: {}", self.path));
            return false;
        }

        self.load_font_info();
        self.loaded = true;
        true
    }

    fn load_font_info(&mut self) {
        let img = self.loader.img.as_ref().unwrap();
        if self.loader.char_width != 0 {
            self.char_width = self.loader.char_width;
            self.char_height = self.loader.char_height;
        } else {
            self.char_width = img.width() as u32 / 16;
            self.char_height = img.height() as u32 / 16;
        }
        self.img_width = img.width() as u32;
        self.img_height = img.height() as u32;

        self.len = (self.img_width / self.char_width) * (self.img_height / self.char_height);

        crate::app::App::print(format!(
            "Font loaded: {} -> font size: {:?} char size: {:?} len: {:?}",
            self.path.as_str(),
            (self.img_width, self.img_height),
            (self.char_width, self.char_height),
            self.len()
        ));
    }
}
