use super::context::{AppContext, ContextImpl};
use super::input::AppInput;
use crate::{AppConfig, AppEvent, Screen, ScreenCreator, ScreenResult};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type GlyphMap = HashMap<String, u32>;

// fps
const MAX_FRAMESKIP: i32 = 5;
const TICKS_PER_SECOND: f64 = 60.0;
const SKIP_TICKS: f64 = 1.0 / TICKS_PER_SECOND;

// default options
pub const DEFAULT_CONSOLE_WIDTH: u32 = 80;
pub const DEFAULT_CONSOLE_HEIGHT: u32 = 45;

/// What is returned by the [`Engine::update`] function
pub enum RunnerEvent {
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
    app: Option<crate::app::App>,
    // gl: uni_gl::WebGLRenderingContext,
    config: AppConfig,
    fps: Fps,
    api: ContextImpl,
    screens: Vec<Box<dyn Screen>>,
    screen_resolution: (u32, u32),
    ready: bool,
}

impl Runner {
    pub fn new(mut options: AppConfig, fonts: Vec<String>) -> Self {
        options.headless = false;
        let app = crate::app::App::new(options.clone());

        let real_screen_width = (options.size.0 as f32 * app.hidpi_factor()) as u32;
        let real_screen_height = (options.size.1 as f32 * app.hidpi_factor()) as u32;

        let gl = uni_gl::WebGLRenderingContext::new(app.canvas());
        let screen_resolution = app.get_screen_resolution();
        let (x_offset, y_offset) = if options.fullscreen && cfg!(not(target_arch = "wasm32")) {
            let x_offset = (screen_resolution.0 - real_screen_width) as i32 / 2;
            let y_offset = (screen_resolution.1 - real_screen_height) as i32 / 2;
            (x_offset, y_offset)
        } else {
            (0, 0)
        };
        crate::app::App::print(format!(
            "Screen size {} x {} offset {} x {} GL viewport : {} x {}  hidpi factor : {}",
            options.size.0,
            options.size.1,
            x_offset,
            y_offset,
            real_screen_width,
            real_screen_height,
            app.hidpi_factor()
        ));
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

        // TODO this should be handled in uni-app
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

        // let font = Font::new(&options.font_path);

        // let mut con = Console::new(0, options.console_width, options.console_height, font, &gl);
        // let extents = options.console_extents;
        // con.set_extents(extents.0, extents.1, extents.2, extents.3);

        // con.set_glyphs(&options.glyphs);

        let mut api = ContextImpl {
            input,
            // cons: vec![con],
            fps: 0,
            average_fps: 0,
            screen_size: options.size.clone(),
            frame_time_ms: 0.0,
            fonts: HashMap::new(),
            gl,
            ready: false,
        };

        for font in fonts {
            api.load_font(&font);
        }
        crate::App::print("Runner created");

        Self {
            api,
            app: Some(app),
            config: options,
            fps: Fps::new(),
            screens: Vec::new(),
            screen_resolution,
            ready: false,
        }
    }

    fn push(&mut self, mut screen: Box<dyn Screen>) {
        screen.setup(&mut self.api);
        if screen.is_full_screen() {
            // ctx.clear_all();
        }
        self.screens.push(screen);
    }

    // pub fn set_engine(&mut self, engine: Box<dyn Engine>) {
    //     self.engine = Some(engine);
    // }

    // pub fn add_console(&mut self, width: u32, height: u32, font_path: &str) -> &mut Console {
    //     let idx = self.api.cons.len() as u32;
    //     let font = Font::new(font_path);
    //     let con = Console::new(idx, width, height, font, &self.gl);
    //     self.api.cons.push(con);
    //     self.api.get_console_mut(idx as usize).unwrap()
    // }

    fn resize(&mut self, hidpi_factor: f32, (real_screen_width, real_screen_height): (u32, u32)) {
        let (x_offset, y_offset) = if self.config.fullscreen && cfg!(not(target_arch = "wasm32")) {
            let x_offset = (self.screen_resolution.0 - real_screen_width) as i32 / 2;
            let y_offset = (self.screen_resolution.1 - real_screen_height) as i32 / 2;
            (x_offset, y_offset)
        } else {
            (0, 0)
        };
        self.api
            .gl
            .viewport(x_offset, y_offset, real_screen_width, real_screen_height);
        self.api.resize(
            (real_screen_width as f32 / hidpi_factor) as u32,
            (real_screen_height as f32 / hidpi_factor) as u32,
        );

        // engine.resize(&mut self.api);
        for screen in self.screens.iter_mut() {
            screen.resize(&mut self.api);
        }

        // let con_size = self.api.con().get_size();
        if cfg!(target_arch = "wasm32") {
            self.api.input.resize(
                self.config.size,
                // con_size,
                (x_offset as u32, y_offset as u32),
            )
        } else {
            self.api.input.resize(
                (real_screen_width, real_screen_height),
                // con_size,
                (x_offset as u32, y_offset as u32),
            )
        };
    }

    fn handle_event(&mut self, ev: &AppEvent) -> Option<RunnerEvent> {
        let ctx = &mut self.api;
        ctx.input.on_event(ev);
        if let Some(mode) = self.screens.last_mut() {
            match mode.input(ctx, ev) {
                ScreenResult::Continue => (),
                ScreenResult::Pop => {
                    // ctx.clear_all();
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
                    // ctx.clear_all();
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
        None
    }

    pub fn run<T>(mut self)
    where
        T: ScreenCreator,
    {
        // self.api.set_font_path(&self.options.font_path);
        let app = self.app.take().unwrap();

        let mut next_tick: f64 = crate::app::now();
        let mut next_frame = next_tick;

        app.run(move |app: &mut crate::app::App| {
            if !self.ready {
                if self.api.ready() {
                    let mut screen = T::create(&mut self.api);
                    screen.setup(&mut self.api);
                    self.screens.push(screen);

                    self.ready = true;
                    crate::App::print("Runner ready");
                }
            } else {
                // self.handle_input(&mut screen, app.hidpi_factor(), app.events.clone());

                if let Some(event) = self.handle_input(app.hidpi_factor(), app.events.clone()) {
                    match event {
                        RunnerEvent::Capture(filepath) => capture_screen(
                            &self.api.gl,
                            self.config.size.0 * app.hidpi_factor() as u32,
                            self.config.size.1 * app.hidpi_factor() as u32,
                            &filepath,
                        ),
                        RunnerEvent::Exit => crate::app::App::exit(),
                        RunnerEvent::Next => {}
                    }
                }

                let mut skipped_frames: i32 = -1;
                let time = crate::app::now();
                while time > next_tick && skipped_frames < MAX_FRAMESKIP {
                    self.api.frame_time_ms = SKIP_TICKS as f32 * 1000.0; // TODO - Use real elapsed time?
                    if let Some(event) = self.update() {
                        match event {
                            RunnerEvent::Capture(filepath) => capture_screen(
                                &self.api.gl,
                                self.config.size.0,
                                self.config.size.1,
                                &filepath,
                            ),
                            RunnerEvent::Exit => crate::app::App::exit(),
                            RunnerEvent::Next => {}
                        }
                    }
                    next_tick += SKIP_TICKS;
                    skipped_frames += 1;
                    self.api.input.on_frame();
                }
                if skipped_frames == MAX_FRAMESKIP {
                    next_tick = time + SKIP_TICKS;
                }
                if self.config.fps == 0 || time > next_frame {
                    self.render();
                    self.fps.step();
                    self.api.fps = self.fps.fps();
                    self.api.average_fps = self.fps.average();

                    // self.gl.clear(uni_gl::BufferBit::Color);
                    // self.gl.clear(uni_gl::BufferBit::Depth); // If using ZPos
                    // self.api.render(&self.gl);
                    if self.config.fps > 0 {
                        next_frame += 1.0 / self.config.fps as f64;
                    }
                }
            }
        });
    }

    fn update(&mut self) -> Option<RunnerEvent> {
        let frame_time_ms = self.api.frame_time_ms();
        if let Some(mode) = self.screens.last_mut() {
            match mode.update(&mut self.api, frame_time_ms) {
                ScreenResult::Continue => (),
                ScreenResult::Pop => {
                    // ctx.clear_all();
                    mode.teardown(&mut self.api);
                    self.screens.pop();
                    match self.screens.last_mut() {
                        Some(m) => m.resume(&mut self.api),
                        _ => {}
                    }
                }
                ScreenResult::Replace(next) => {
                    // ctx.clear_all();
                    mode.teardown(&mut self.api);
                    self.screens.pop();
                    self.push(next);
                }
                ScreenResult::Push(next) => {
                    mode.pause(&mut self.api);
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

        // TODO - convert to using buffer
        for screen in self.screens.iter_mut().skip(start_idx) {
            screen.render(&mut self.api);
        }
    }
}

/// This captures an in-game screenshot and saves it to the file
fn capture_screen(gl: &uni_gl::WebGLRenderingContext, w: u32, h: u32, filepath: &str) {
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

    if cfg!(not(target_arch = "wasm32")) {
        // disabled on wasm target
        image::save_buffer(
            filepath,
            &image::imageops::flip_vertical(&img),
            w,
            h,
            image::ColorType::Rgba8,
        )
        .expect("Failed to save buffer to the specified path");
    }
}

// pub(crate) fn create_texture(gl: &uni_gl::WebGLRenderingContext) -> uni_gl::WebGLTexture {
//     let tex = gl.create_texture();
//     gl.active_texture(0);
//     gl.bind_texture(&tex);
//     set_texture_params(gl, true);
//     tex
// }

pub struct Fps {
    counter: u32,
    start: f64,
    last: f64,
    total_frames: u64,
    fps: u32,
    average: u32,
}

impl Fps {
    pub fn new() -> Fps {
        let now = crate::app::now();
        Fps {
            counter: 0,
            total_frames: 0,
            start: now,
            last: now,
            fps: 0,
            average: 0,
        }
    }
    pub fn fps(&self) -> u32 {
        self.fps
    }

    pub fn step(&mut self) {
        self.counter += 1;
        self.total_frames += 1;
        let curr = crate::app::now();
        if curr - self.last > 1.0 {
            self.last = curr;
            self.fps = self.counter;
            self.counter = 0;
            self.average = (self.total_frames as f64 / (self.last - self.start)) as u32;
        }
    }
    pub fn average(&self) -> u32 {
        self.average
    }
}
