use crate::file::FileLoader;

pub struct FontLoader {
    loader: FileLoader,
    pub img: Option<image::RgbaImage>,
    id: usize,
}

impl FontLoader {
    pub fn new() -> Self {
        Self {
            loader: FileLoader::new(),
            img: None,
            id: 0,
        }
    }
    pub fn load_font(&mut self, path: &str) {
        match self.loader.load_file(path) {
            Ok(id) => {
                self.id = id;
                self.load_font_async();
            }
            Err(msg) => {
                panic!("Error while loading file {} : {}", path, msg);
            }
        }
    }

    pub fn load_font_async(&mut self) -> bool {
        if self.img.is_some() {
            return true;
        }
        if self.loader.check_file_ready(self.id) {
            let buf = self.loader.get_file_content(self.id);
            let mut img = image::load_from_memory(&buf).unwrap().to_rgba8();
            process_image(&mut img);
            self.img = Some(img);
            return true;
        }
        false
    }

    pub fn load_bytes(&mut self, buf: &[u8]) {
        let mut img = image::load_from_memory(buf).unwrap().to_rgba8();
        process_image(&mut img);
        self.img = Some(img);
    }
}

fn process_image(img: &mut image::RgbaImage) {
    let pixel = img.get_pixel(0, 0);
    let alpha = pixel[3];
    if alpha == 255 {
        let transparent_color = (pixel[0], pixel[1], pixel[2]);
        let greyscale = transparent_color == (0, 0, 0);
        crate::console(&format!(
            "{}transparent color: {:?}",
            if greyscale { "greyscale " } else { "" },
            transparent_color
        ));
        let (width, height) = img.dimensions();
        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel_mut(x, y);
                if (pixel[0], pixel[1], pixel[2]) == transparent_color {
                    pixel[3] = 0;
                    pixel[0] = 0;
                    pixel[1] = 0;
                    pixel[2] = 0;
                } else if greyscale && pixel[0] == pixel[1] && pixel[1] == pixel[2] {
                    let alpha = pixel[0];
                    pixel[0] = 255;
                    pixel[1] = 255;
                    pixel[2] = 255;
                    pixel[3] = alpha;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::SUBCELL_BYTES;

    #[test]
    fn subcell_font() {
        let mut loader = FontLoader::new();
        loader.load_bytes(SUBCELL_BYTES);

        assert!(loader.img.is_some());
        assert!(loader.load_font_async());
    }
}
