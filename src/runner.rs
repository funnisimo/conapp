use super::context::AppContext;
use super::input::AppInput;
use crate::{
    console, App, AppConfig, AppEvent, LoadCallback, LoadError, LoadingScreen, Screen, ScreenResult,
};
use std::cell::RefCell;
use std::rc::Rc;

/// What is returned by the internal update and input functions
enum RunnerEvent {
    /// Save a screenshot. parameter = file path.
    /// The file name must have a .png extension.
    /// This is ignored on WASM platform.
    Capture(String),
    /// end the program
    Exit,
    /// Skip to next stage of processing (input->update->render)
    Next,
}

/// This is the game application. It handles the creation of the game window, the window events including player input events and runs the main game loop.
pub struct Runner {
    /// The uni_gl::App that controls the window
    app: Option<crate::app::App>,
    /// All of the configuration settings
    config: AppConfig,
    /// Maximum number of update calls to do in one frame
    max_frameskip: i32,

    app_ctx: AppContext,
    screens: Vec<Box<dyn Screen>>,
    screen_resolution: (u32, u32),
    real_screen_size: (u32, u32),
    ready: bool,
}

impl Runner {
    pub fn new(mut options: AppConfig) -> Self {
        options.headless = false;
        let app = crate::app::App::new(options.clone());

        let real_screen_width = (options.size.0 as f32 * app.hidpi_factor()) as u32;
        let real_screen_height = (options.size.1 as f32 * app.hidpi_factor()) as u32;

        let screen_resolution = app.screen_resolution();

        crate::console("Runner created");

        Self {
            app_ctx: create_ctx(&app, &options),
            app: Some(app),
            config: options,
            max_frameskip: 5,
            screens: Vec::new(),
            screen_resolution,
            real_screen_size: (real_screen_width, real_screen_height),
            ready: false,
        }
    }

    fn push(&mut self, mut screen: Box<dyn Screen>) {
        screen.setup(&mut self.app_ctx);
        if screen.is_full_screen() {
            self.app_ctx.clear(None);
        }
        self.screens.push(screen);
    }

    pub fn load_file(&mut self, path: &str, cb: Box<LoadCallback>) -> Result<(), LoadError> {
        self.app_ctx.load_file(path, cb)
    }

    pub fn load_font(&mut self, font_path: &str) -> Result<(), LoadError> {
        self.app_ctx.load_font(font_path)
    }

    pub fn load_image(&mut self, image_path: &str) -> Result<(), LoadError> {
        self.app_ctx.load_image(image_path)
    }

    fn resize(&mut self, hidpi_factor: f32, (real_screen_width, real_screen_height): (u32, u32)) {
        console(format!(
            "runner::resize - {}x{}, hidpi={}",
            real_screen_width, real_screen_height, hidpi_factor
        ));

        let (x_offset, y_offset) = if self.config.fullscreen && cfg!(not(target_arch = "wasm32")) {
            let x_offset = (self.screen_resolution.0 - real_screen_width) as i32 / 2;
            let y_offset = (self.screen_resolution.1 - real_screen_height) as i32 / 2;
            (x_offset, y_offset)
        } else {
            (0, 0)
        };
        self.real_screen_size = (real_screen_width, real_screen_height);

        self.app_ctx
            .gl
            .viewport(x_offset, y_offset, real_screen_width, real_screen_height);
        self.app_ctx.resize(
            (real_screen_width as f32 / hidpi_factor) as u32,
            (real_screen_height as f32 / hidpi_factor) as u32,
        );

        // engine.resize(&mut self.api);
        for screen in self.screens.iter_mut() {
            screen.resize(&mut self.app_ctx);
        }

        // let con_size = self.api.con().size();
        if cfg!(target_arch = "wasm32") {
            self.app_ctx.input.resize(
                self.config.size,
                // con_size,
                (x_offset as u32, y_offset as u32),
            )
        } else {
            self.app_ctx.input.resize(
                (real_screen_width, real_screen_height),
                // con_size,
                (x_offset as u32, y_offset as u32),
            )
        };
    }

    fn handle_event(&mut self, ev: &AppEvent) -> Option<RunnerEvent> {
        let ctx = &mut self.app_ctx;
        ctx.input.on_event(ev);
        if let Some(mode) = self.screens.last_mut() {
            match mode.input(ctx, ev) {
                ScreenResult::Continue => (),
                ScreenResult::Capture(name) => return Some(RunnerEvent::Capture(name)),
                ScreenResult::Pop => {
                    ctx.clear(None);
                    mode.teardown(ctx);
                    self.screens.pop();
                    match self.screens.last_mut() {
                        Some(m) => m.resume(ctx),
                        _ => {}
                    }
                    // self.render(ctx);
                    return Some(RunnerEvent::Next);
                }
                ScreenResult::Replace(next) => {
                    ctx.clear(None);
                    mode.teardown(ctx);
                    self.screens.pop();
                    self.push(next);
                    // self.render(ctx);
                    return Some(RunnerEvent::Next);
                }
                ScreenResult::Push(next) => {
                    mode.pause(ctx);
                    self.push(next);
                    // self.render(ctx);
                    return Some(RunnerEvent::Next);
                }
                ScreenResult::Quit => {
                    return Some(RunnerEvent::Exit);
                }
            }
        }
        None
    }

    fn handle_input(
        &mut self,
        hidpi_factor: f32,
        events: Rc<RefCell<Vec<crate::app::AppEvent>>>,
    ) -> Option<RunnerEvent> {
        for evt in events.borrow().iter() {
            if let crate::app::AppEvent::Resized(size) = evt {
                self.resize(hidpi_factor, *size);
            } else {
                if let Some(ev) = self.handle_event(evt) {
                    return Some(ev);
                }
            }
        }
        // if self.app_ctx.input().had_mouse_event() {
        //     let mpos = self.app_ctx.input.mouse_pct();
        //     if let Some(ev) = self.handle_event(&AppEvent::MousePos(mpos)) {
        //         return Some(ev);
        //     }
        // }
        None
    }

    pub fn run_screen(self, screen: Box<dyn Screen>) {
        self.run_with(Box::new(|_| screen))
    }

    pub fn run_with(mut self, func: Box<dyn FnOnce(&mut AppContext) -> Box<dyn Screen>>) {
        // self.api.set_font_path(&self.options.font_path);
        let app = self.app.take().unwrap();

        let mut last_frame_time = crate::app::perf_now();
        let mut next_frame = last_frame_time;

        let mut screen = match self.app_ctx.has_files_to_load() {
            false => func(&mut self.app_ctx),
            true => {
                console("Using loading screen");
                LoadingScreen::new(func)
            }
        };
        // let mut screen = LoadingScreen::new(func);
        screen.setup(&mut self.app_ctx);
        self.screens.push(screen);

        self.ready = true;
        crate::console(format!("Runner ready"));

        app.run(move |app: &mut crate::app::App| {
            self.app_ctx.load_files(); // Do any font/image loading necessary

            if self.screens.is_empty() {
                return crate::app::App::exit();
            }

            if let Some(event) = self.handle_input(app.hidpi_factor(), app.events.clone()) {
                match event {
                    RunnerEvent::Capture(filepath) => {
                        capture_screen(
                            &self.app_ctx.gl,
                            self.real_screen_size.0,
                            // self.screen_resolution.0 * app.hidpi_factor() as u32,
                            self.real_screen_size.1,
                            // self.screen_resolution.1 * app.hidpi_factor() as u32,
                            &filepath,
                        )
                    }
                    RunnerEvent::Exit => {
                        console("App Exit");
                        crate::app::App::exit();
                    }
                    RunnerEvent::Next => {}
                }
            }

            let mut skipped_frames: i32 = -1;
            let time = crate::app::perf_now();
            let skip_ticks = match self.app_ctx.fps.goal() {
                0 => time - last_frame_time,
                x => 1.0 / x as f64,
            };

            while time >= last_frame_time && skipped_frames < self.max_frameskip {
                // self.app_ctx.frame_time_ms = SKIP_TICKS as f32 * 1000.0; // TODO - Use real elapsed time?
                self.app_ctx.frame_time_ms = skip_ticks * 1000.0; // TODO - Use real elapsed time?
                if let Some(event) = self.update() {
                    match event {
                        RunnerEvent::Capture(filepath) => capture_screen(
                            &self.app_ctx.gl,
                            self.real_screen_size.0,
                            // self.screen_resolution.0 * app.hidpi_factor() as u32,
                            self.real_screen_size.1,
                            // self.screen_resolution.1 * app.hidpi_factor() as u32,
                            &filepath,
                        ),
                        RunnerEvent::Exit => crate::app::App::exit(),
                        RunnerEvent::Next => {}
                    }
                }
                last_frame_time += skip_ticks;
                // next_tick += SKIP_TICKS;
                skipped_frames += 1;
            }
            self.app_ctx.input.on_frame_end();
            if skipped_frames == self.max_frameskip {
                // next_tick = time + SKIP_TICKS;
                last_frame_time = time + skip_ticks;
            }
            if self.app_ctx.fps.goal() == 0 || time >= next_frame {
                self.render();
                self.app_ctx.fps.step();

                if self.app_ctx.fps.goal() > 0 {
                    next_frame += 1.0 / self.app_ctx.fps.goal() as f64;
                }
            }
            // }
        });
    }

    fn update(&mut self) -> Option<RunnerEvent> {
        let frame_time_ms = self.app_ctx.frame_time_ms();
        if let Some(mode) = self.screens.last_mut() {
            match mode.update(&mut self.app_ctx, frame_time_ms) {
                ScreenResult::Continue => (),
                ScreenResult::Capture(name) => return Some(RunnerEvent::Capture(name)),
                ScreenResult::Pop => {
                    self.app_ctx.clear(None);
                    mode.teardown(&mut self.app_ctx);
                    self.screens.pop();
                    match self.screens.last_mut() {
                        Some(m) => m.resume(&mut self.app_ctx),
                        _ => {}
                    }
                }
                ScreenResult::Replace(next) => {
                    self.app_ctx.clear(None);
                    mode.teardown(&mut self.app_ctx);
                    self.screens.pop();
                    self.push(next);
                }
                ScreenResult::Push(next) => {
                    mode.pause(&mut self.app_ctx);
                    self.push(next);
                }
                ScreenResult::Quit => {
                    return Some(RunnerEvent::Exit);
                }
            }
        }
        None
    }

    /// This is called before drawing the console on the screen. The framerate depends on the screen frequency, the graphic cards and on whether you activated vsync or not.
    /// The framerate is not reliable so don't update time related stuff in this function.
    /// The screen will display the content of the root console provided by `api.con()`
    fn render(&mut self) {
        // Find last full screen mode (that is where we start drawing)
        let mut start_idx = 0;
        for (idx, m) in self.screens.iter().enumerate() {
            if m.is_full_screen() {
                start_idx = idx;
            }
        }
        self.app_ctx.clear(None);
        for screen in self.screens.iter_mut().skip(start_idx) {
            screen.render(&mut self.app_ctx);
        }
    }
}

fn create_ctx(app: &App, options: &AppConfig) -> AppContext {
    let real_screen_width = (options.size.0 as f32 * app.hidpi_factor()) as u32;
    let real_screen_height = (options.size.1 as f32 * app.hidpi_factor()) as u32;

    let screen_resolution = app.screen_resolution();
    let (x_offset, y_offset) = if options.fullscreen && cfg!(not(target_arch = "wasm32")) {
        let x_offset = (screen_resolution.0 - real_screen_width) as i32 / 2;
        let y_offset = (screen_resolution.1 - real_screen_height) as i32 / 2;
        (x_offset, y_offset)
    } else {
        (0, 0)
    };
    crate::console(&format!(
        "Screen size {} x {} offset {} x {} GL viewport : {} x {}  hidpi factor : {}",
        options.size.0,
        options.size.1,
        x_offset,
        y_offset,
        real_screen_width,
        real_screen_height,
        app.hidpi_factor()
    ));

    let gl = uni_gl::WebGLRenderingContext::new(app.canvas());
    gl.viewport(x_offset, y_offset, real_screen_width, real_screen_height);
    gl.enable(uni_gl::Flag::Blend as i32);
    // gl.enable(uni_gl::Flag::DepthTest as i32);   // If using ZPos
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(uni_gl::BufferBit::Color);
    // gl.clear(uni_gl::BufferBit::Depth);  // If using ZPos
    gl.blend_equation(uni_gl::BlendEquation::FuncAdd);
    gl.blend_func(
        uni_gl::BlendMode::SrcAlpha,
        uni_gl::BlendMode::OneMinusSrcAlpha,
    );

    let input = if cfg!(target_arch = "wasm32") {
        AppInput::new(
            (options.size.0, options.size.1),
            // (options.console_width, options.console_height),
            (x_offset as u32, y_offset as u32),
        )
    } else {
        AppInput::new(
            (real_screen_width, real_screen_height),
            // (options.console_width, options.console_height),
            (x_offset as u32, y_offset as u32),
        )
    };

    AppContext::new(gl, options.size.clone(), input, options.fps)
}

/// This captures an in-game screenshot and saves it to the file
fn capture_screen(gl: &uni_gl::WebGLRenderingContext, w: u32, h: u32, filepath: &str) {
    if cfg!(not(target_arch = "wasm32")) {
        let mut img = image::DynamicImage::new_rgba8(w, h);
        let pixels = img.as_mut_rgba8().unwrap();

        gl.pixel_storei(uni_gl::PixelStorageMode::PackAlignment, 1);
        gl.read_pixels(
            0,
            0,
            w,
            h,
            uni_gl::PixelFormat::Rgba,
            uni_gl::PixelType::UnsignedByte,
            pixels,
        );

        // disabled on wasm target
        image::save_buffer(
            filepath,
            &image::imageops::flip_vertical(&img),
            w,
            h,
            image::ColorType::Rgba8,
        )
        .expect("Failed to save buffer to the specified path");
    } else {
        crate::console("Screen capture not supported on web platform");
    }
}
