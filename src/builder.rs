use crate::{AppConfig, Runner};

/// Builds an application runner
pub struct AppBuilder {
    /// window configuration info
    config: AppConfig,
    /// fonts to load
    fonts: Vec<String>,
    /// images to load
    images: Vec<String>,
    /// fps goal for application
    fps_goal: u32,
}

impl AppBuilder {
    /// Starts building a [`Runner`] with the given screen width and height in pixels
    pub fn new(width: u32, height: u32) -> Self {
        let options = AppConfig::new("application", (width, height));
        AppBuilder {
            config: options,
            fonts: Vec::new(),
            images: Vec::new(),
            fps_goal: 60,
        }
    }

    /// Sets the window title
    pub fn title(mut self, title: &str) -> Self {
        self.config.title = title.to_owned();
        self
    }

    /// Turns on/off the vsync
    pub fn vsync(mut self, val: bool) -> Self {
        self.config.vsync = val;
        self
    }

    /// Makes the application run headless
    pub fn headless(mut self, val: bool) -> Self {
        self.config.headless = val;
        self
    }

    /// Run fullscreen?
    pub fn fullscreen(mut self, val: bool) -> Self {
        self.config.fullscreen = val;
        self
    }

    /// Resizable?
    pub fn resizable(mut self, val: bool) -> Self {
        self.config.resizable = val;
        self
    }

    /// Show the mouse cursor?
    pub fn show_cursor(mut self, val: bool) -> Self {
        self.config.show_cursor = val;
        self
    }

    /// If on, clicking the close button on the window creates an event that you can handle
    pub fn intercept_close_request(mut self, val: bool) -> Self {
        self.config.intercept_close_request = val;
        self
    }

    /// Loads a font on startup
    pub fn font(mut self, font_path: &str) -> Self {
        self.fonts.push(font_path.to_owned());
        self
    }

    /// Loads an image on startup
    pub fn image(mut self, image_path: &str) -> Self {
        self.images.push(image_path.to_owned());
        self
    }

    /// Sets the fps goal
    pub fn fps(mut self, fps_goal: u32) -> Self {
        self.fps_goal = fps_goal;
        self
    }

    /// Builds the [`Runner`]
    pub fn build(self) -> Runner {
        let mut runner = Runner::new(self.config, self.fps_goal);
        for font in self.fonts {
            runner.load_font(&font);
        }
        for image in self.images {
            runner.load_image(&image).expect("Failed to load image.");
        }
        runner
    }
}
