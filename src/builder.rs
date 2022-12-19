use crate::{AppConfig, Runner};

pub struct AppBuilder {
    config: AppConfig,
    fonts: Vec<String>,
}

impl AppBuilder {
    pub fn new(width: u32, height: u32) -> Self {
        let options = AppConfig::new("application", (width, height));
        AppBuilder {
            config: options,
            fonts: Vec::new(),
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.config.title = title.to_owned();
        self
    }
    pub fn vsync(mut self, val: bool) -> Self {
        self.config.vsync = val;
        self
    }
    pub fn headless(mut self, val: bool) -> Self {
        self.config.headless = val;
        self
    }
    pub fn fullscreen(mut self, val: bool) -> Self {
        self.config.fullscreen = val;
        self
    }
    pub fn resizable(mut self, val: bool) -> Self {
        self.config.resizable = val;
        self
    }
    pub fn show_cursor(mut self, val: bool) -> Self {
        self.config.show_cursor = val;
        self
    }
    pub fn intercept_close_request(mut self, val: bool) -> Self {
        self.config.intercept_close_request = val;
        self
    }

    pub fn font(mut self, font_path: &str) -> Self {
        self.fonts.push(font_path.to_owned());
        self
    }

    pub fn build(self) -> Runner {
        Runner::new(self.config, self.fonts)
    }
}
