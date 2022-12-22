use crate::{AppConfig, AppContext, Runner};

pub struct AppBuilder {
    config: AppConfig,
    fonts: Vec<String>,
    fps_goal: u32,
}

impl AppBuilder {
    pub fn new(width: u32, height: u32) -> Self {
        let options = AppConfig::new("application", (width, height));
        AppBuilder {
            config: options,
            fonts: Vec::new(),
            fps_goal: 0,
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

    pub fn fps(mut self, fps_goal: u32) -> Self {
        self.fps_goal = fps_goal;
        self
    }

    pub fn build(self) -> Runner {
        let mut runner = Runner::new(self.config, self.fps_goal);
        for font in self.fonts {
            runner.app_ctx.load_font(&font);
        }
        runner
    }
}
