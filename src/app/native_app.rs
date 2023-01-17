mod native_keycode;

use crate::console;

use self::native_keycode::{translate_scan_code, translate_virtual_key};
use super::events;
use super::AppConfig;
use super::AppEvent;
use super::{File, FileSystem};
use glutin::config::{Config, ConfigTemplateBuilder};
use glutin::context::NotCurrentContext;
use glutin::context::PossiblyCurrentContext;
use glutin::context::{ContextApi, ContextAttributesBuilder};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributesBuilder, SwapInterval, WindowSurface};
use glutin_winit::{self, DisplayBuilder};
use raw_window_handle::HasRawWindowHandle;
use std::cell::RefCell;
use std::env;
use std::ffi::CString;
use std::num::NonZeroU32;
use std::os::raw::c_void;
use std::process;
use std::rc::Rc;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use winit::dpi::LogicalSize;
use winit::event::ElementState;
use winit::event::Event;
use winit::event::KeyboardInput;
use winit::event::ModifiersState;
use winit::event::MouseButton;
use winit::event::VirtualKeyCode;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::event_loop::EventLoopBuilder;
use winit::event_loop::EventLoopWindowTarget;
use winit::monitor::VideoMode;
use winit::window::Fullscreen;
use winit::window::Window;
use winit::window::WindowBuilder;

// enum WindowContext {
//     Headless(Context<PossiblyCurrent>),
//     Normal(WindowedContext<PossiblyCurrent>),
// }

// impl WindowContext {
//     fn hidpi_factor(&self) -> f32 {
//         match self {
//             WindowContext::Normal(ref w) => w.window().scale_factor() as f32,
//             _ => 1.0,
//         }
//     }

//     fn window(&self) -> &WindowedContext<PossiblyCurrent> {
//         match self {
//             WindowContext::Normal(ref w) => w,
//             _ => unimplemented!(),
//         }
//     }

//     fn context(&self) -> &Context<PossiblyCurrent> {
//         match self {
//             WindowContext::Normal(w) => w.context(),
//             WindowContext::Headless(w) => w,
//         }
//     }

//     fn swap_buffers(&self) -> Result<(), glutin::ContextError> {
//         match self {
//             WindowContext::Normal(ref w) => w.swap_buffers(),
//             WindowContext::Headless(_) => Ok(()),
//         }
//     }
// }

struct InputState {
    modifiers: ModifiersState,
    mouse_pos: (f32, f32),
    hidpi: f32,
}

impl InputState {
    fn new() -> Self {
        InputState {
            modifiers: ModifiersState::empty(),
            mouse_pos: (0.0, 0.0),
            hidpi: 1.0,
        }
    }

    fn shift(&self) -> bool {
        self.modifiers.shift()
    }

    fn alt(&self) -> bool {
        self.modifiers.alt()
    }

    fn ctrl(&self) -> bool {
        self.modifiers.ctrl()
    }

    fn logo(&self) -> bool {
        self.modifiers.logo()
    }
}

pub struct GlWindow {
    // XXX the surface must be dropped before the window.
    pub surface: Surface<WindowSurface>,

    pub window: Window,
}

impl GlWindow {
    pub fn new(window: Window, config: &Config) -> Self {
        let (width, height): (u32, u32) = window.inner_size().into();
        let raw_window_handle = window.raw_window_handle();
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let surface = unsafe {
            config
                .display()
                .create_window_surface(config, &attrs)
                .unwrap()
        };

        Self { window, surface }
    }
}

/// the main application struct
pub struct App {
    not_current_gl_context: Option<NotCurrentContext>,
    gl_config: Config,
    window: Option<Window>,
    state: Option<(PossiblyCurrentContext, GlWindow)>,

    // window: WindowContext,
    events_loop: Option<EventLoop<()>>,
    intercept_close_request: bool,
    input_state: InputState,
    // pub events: Rc<RefCell<Vec<AppEvent>>>,
    pub events: Rc<RefCell<Vec<AppEvent>>>,
    dropped_files: Vec<File>,
    fullscreen_resolution: VideoMode,
}

impl App {
    /// create a new game window
    pub fn new(config: AppConfig) -> App {
        let events_loop = EventLoopBuilder::new().build();

        let fullscreen_resolution = events_loop
            .available_monitors()
            .nth(0)
            .unwrap()
            .video_modes()
            .nth(0)
            .unwrap();

        // Only windows requires the window to be present before creating the display.
        // Other platforms don't really need one.
        //
        // XXX if you don't care about running on android or so you can safely remove
        // this condition and always pass the window builder.
        let window_builder = Some(
            WindowBuilder::new()
                .with_inner_size(LogicalSize::new(config.size.0, config.size.1))
                .with_transparent(true),
        );

        // The template will match only the configurations supporting rendering to
        // windows.
        let template = ConfigTemplateBuilder::new().with_alpha_size(8);

        let display_builder = DisplayBuilder::new().with_window_builder(window_builder);

        let (window, gl_config) = display_builder
            .build(&events_loop, template, |configs| {
                // Find the config with the maximum number of samples, so our triangle will
                // be smooth.
                configs
                    .reduce(|accum, config| {
                        let transparency_check = config.supports_transparency().unwrap_or(false)
                            & !accum.supports_transparency().unwrap_or(false);

                        if transparency_check || config.num_samples() > accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        println!("Picked a config with {} samples", gl_config.num_samples());

        let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());

        // XXX The display could be obtained from the any object created by it, so we
        // can query it from the config.
        let gl_display = gl_config.display();

        // The context creation part. It can be created before surface and that's how
        // it's expected in multithreaded + multiwindow operation mode, since you
        // can send NotCurrentContext, but not Surface.
        let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

        // Since glutin by default tries to create OpenGL core context, which may not be
        // present we should try gles.
        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(raw_window_handle);
        let not_current_gl_context = Some(unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    gl_display
                        .create_context(&gl_config, &fallback_context_attributes)
                        .expect("failed to create context")
                })
        });

        App {
            gl_config,
            not_current_gl_context,
            window,
            state: None,

            events_loop: Some(events_loop),
            intercept_close_request: config.intercept_close_request,
            events: Rc::new(RefCell::new(Vec::new())),
            dropped_files: Vec::new(),
            input_state: InputState::new(),
            fullscreen_resolution,
        }
    }

    /// return the screen resolution in physical pixels
    pub fn screen_resolution(&self) -> (u32, u32) {
        // if let WindowContext::Normal(ref glwindow) = self.window {
        //     if let Some(ref monitor) = glwindow.window().current_monitor() {
        //         return monitor.size().into();
        //     }
        // }
        // (0, 0)

        if let Some((_, gl_window)) = &self.state {
            if let Some(ref monitor) = gl_window.window.current_monitor() {
                return monitor.size().into();
            }
        }
        (0, 0)
    }

    pub fn viewport_size(&self) -> (u32, u32) {
        if let Some((_, gl_window)) = &self.state {
            let dpi = self.hidpi_factor();
            let size = gl_window.window.inner_size();
            return (
                (size.width as f32 / dpi) as u32,
                (size.height as f32 / dpi) as u32,
            );
        }
        (0, 0)
    }

    /// return the command line / URL parameters
    pub fn params() -> Vec<String> {
        let mut params: Vec<String> = env::args().collect();
        params.remove(0);
        params
    }

    /// activate or deactivate fullscreen. only works on native target
    pub fn set_fullscreen(&mut self, b: bool) {
        if let Some((_, gl_window)) = &self.state {
            if b {
                gl_window.window.set_fullscreen(Some(Fullscreen::Exclusive(
                    self.fullscreen_resolution.clone(),
                )));
            } else {
                gl_window.window.set_fullscreen(None);
            }
        }
    }

    /// print a message on standard output (native) or js console (web)
    pub fn print<T: Into<String>>(msg: T) {
        println!("{}", msg.into());
    }

    /// exit current process (close the game window). On web target, this does nothing.
    pub fn exit() {
        process::exit(0);
    }

    /// returns the HiDPI factor for current screen
    pub fn hidpi_factor(&self) -> f32 {
        if let Some((_, gl_window)) = &self.state {
            return gl_window.window.scale_factor() as f32;
        }
        1.0
    }

    fn proc_address(&self, name: &str) -> *const c_void {
        let gl_display = self.gl_config.display();
        let symbol = CString::new(name).unwrap();
        gl_display.get_proc_address(symbol.as_c_str()).cast()
    }

    /// return the opengl context for this window
    pub fn canvas<'p>(&'p self) -> Box<dyn 'p + FnMut(&str) -> *const c_void> {
        Box::new(move |name| self.proc_address(name))
    }

    fn handle_event(
        &mut self,
        event: Event<()>,
        window_target: &EventLoopWindowTarget<()>,
    ) -> (bool, bool) {
        let mut running = true;
        let mut next_frame = false;
        match event {
            Event::Resumed => {
                #[cfg(target_os = "android")]
                println!("Android window available");

                let window = self.window.take().unwrap_or_else(|| {
                    let window_builder = WindowBuilder::new().with_transparent(true);
                    glutin_winit::finalize_window(window_target, window_builder, &self.gl_config)
                        .unwrap()
                });

                let gl_window = GlWindow::new(window, &self.gl_config);

                // Make it current.
                let gl_context = self
                    .not_current_gl_context
                    .take()
                    .unwrap()
                    .make_current(&gl_window.surface)
                    .unwrap();

                // Try setting vsync.
                if let Err(res) = gl_window
                    .surface
                    .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
                {
                    eprintln!("Error setting vsync: {:?}", res);
                }

                assert!(self.state.replace((gl_context, gl_window)).is_none());
                self.input_state.hidpi = self.hidpi_factor();

                self.events.borrow_mut().push(AppEvent::Ready);
            }
            Event::Suspended => {
                // This event is only raised on Android, where the backing NativeWindow for a GL
                // Surface can appear and disappear at any moment.
                console("Android window removed");

                // Destroy the GL Surface and un-current the GL Context before ndk-glue releases
                // the window back to the system.
                let (gl_context, _) = self.state.take().unwrap();
                assert!(self
                    .not_current_gl_context
                    .replace(gl_context.make_not_current().unwrap())
                    .is_none());

                self.events.borrow_mut().push(AppEvent::Suspended);
            }

            Event::RedrawRequested(_) => {}
            Event::MainEventsCleared => {
                // next_frame = true;
            }
            Event::RedrawEventsCleared => {
                next_frame = true;
            }
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => {
                    if !self.intercept_close_request {
                        running = false;
                    }
                }
                WindowEvent::Resized(size) => {
                    // Fixed for Windows which minimized to emit a Resized(0,0) event
                    if size.width != 0 && size.height != 0 {
                        // Some platforms like EGL require resizing GL surface to update the size
                        // Notable platforms here are Wayland and macOS, other don't require it
                        // and the function is no-op, but it's wise to resize it for portability
                        // reasons.
                        if let Some((gl_context, gl_window)) = &self.state {
                            gl_window.surface.resize(
                                gl_context,
                                NonZeroU32::new(size.width).unwrap(),
                                NonZeroU32::new(size.height).unwrap(),
                            );
                        }
                    }
                }
                WindowEvent::ModifiersChanged(new_state) => {
                    self.input_state.modifiers = *new_state;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    // issue tracked in https://github.com/tomaka/winit/issues/41
                    // Right now we handle it manually.
                    if cfg!(target_os = "macos") {
                        if let Some(keycode) = input.virtual_keycode {
                            if keycode == VirtualKeyCode::Q && self.input_state.logo() {
                                running = false;
                            }
                        }
                    }
                }
                WindowEvent::DroppedFile(ref path) => {
                    let filepath = path.to_str().unwrap();
                    self.dropped_files.push(FileSystem::open(filepath).unwrap());
                }
                _ => (),
            },
            _ => (),
        };

        if let Some(app_event) = translate_event(event, &mut self.input_state) {
            // println!("uni app event - {:?}", app_event);
            let mut ev = self.events.borrow_mut();
            if match app_event {
                AppEvent::CharEvent(ch) => match ev.iter_mut().last() {
                    // eat char events for backspace
                    Some(AppEvent::KeyDown(key_down)) => {
                        key_down.key = ch.to_string();
                        match key_down.key_code {
                            VirtualKeyCode::Back | VirtualKeyCode::Delete => false,
                            _ => true,
                        }
                    }
                    _ => true,
                },
                _ => true,
            } {
                ev.push(app_event);
            }
        }

        (running, next_frame)
    }

    pub fn dropped_file(&mut self) -> Option<File> {
        self.dropped_files.pop()
    }

    /// start the game loop, calling provided callback every frame
    pub fn run<'a, F>(mut self, mut callback: F)
    where
        F: 'static + FnMut(&mut Self) -> (),
    {
        let events_loop = self.events_loop.take().unwrap();
        events_loop.run(move |event, window_target, control_flow| {
            control_flow.set_poll();
            let (running, next_frame) = self.handle_event(event, window_target);
            if !running {
                control_flow.set_exit();
            }
            if next_frame {
                callback(&mut self);
                self.events.borrow_mut().clear();
                if let Some((gl_context, gl_window)) = &self.state {
                    gl_window.window.request_redraw();
                    gl_window.surface.swap_buffers(gl_context).unwrap();
                }
            }
        });
    }
}

fn get_virtual_key(input: KeyboardInput) -> String {
    match input.virtual_keycode {
        Some(k) => {
            let mut s = translate_virtual_key(k).into();
            if s == "" {
                s = format!("{:?}", k);
            }
            s
        }
        None => "".into(),
    }
}

fn get_scan_code(input: KeyboardInput) -> String {
    translate_scan_code(input.scancode & 0xFF).into()
}

fn translate_event(e: Event<()>, input_state: &mut InputState) -> Option<AppEvent> {
    if let Event::WindowEvent {
        event: winevent, ..
    } = e
    {
        match winevent {
            WindowEvent::MouseInput { state, button, .. } => {
                let button_num = match button {
                    MouseButton::Left => 0,
                    MouseButton::Middle => 1,
                    MouseButton::Right => 2,
                    MouseButton::Other(val) => val as usize,
                };
                let event = events::MouseButtonEvent {
                    button: button_num,
                    pos: input_state.mouse_pos,
                };
                match state {
                    ElementState::Pressed => Some(AppEvent::MouseDown(event)),
                    ElementState::Released => Some(AppEvent::MouseUp(event)),
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let pos: (f32, f32) = position.into();
                input_state.mouse_pos = (pos.0 / input_state.hidpi, pos.1 / input_state.hidpi);
                Some(AppEvent::MousePos(input_state.mouse_pos))
            }
            WindowEvent::KeyboardInput { input, .. } => match input.state {
                ElementState::Pressed => Some(AppEvent::KeyDown(events::KeyDownEvent {
                    key: get_virtual_key(input),
                    code: get_scan_code(input),
                    key_code: input.virtual_keycode.unwrap(),
                    shift: input_state.shift(),
                    alt: input_state.alt(),
                    ctrl: input_state.ctrl(),
                })),
                ElementState::Released => Some(AppEvent::KeyUp(events::KeyUpEvent {
                    key: get_virtual_key(input),
                    code: get_scan_code(input),
                    key_code: input.virtual_keycode.unwrap(),
                    shift: input_state.shift(),
                    alt: input_state.alt(),
                    ctrl: input_state.ctrl(),
                })),
            },
            WindowEvent::ReceivedCharacter(c) => Some(AppEvent::CharEvent(c)),
            WindowEvent::Resized(size) => Some(AppEvent::Resized(size.into())),
            WindowEvent::CloseRequested => Some(AppEvent::CloseRequested),
            WindowEvent::DroppedFile(path) => {
                Some(AppEvent::FileDropped(path.to_str().unwrap().to_owned()))
            }
            _ => None,
        }
    } else {
        None
    }
}

/// return the seconds since the epoch
pub fn now() -> f64 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::default())
        .as_secs_f64()
}

/// return a time in secs
pub fn perf_now() -> f64 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::default())
        .as_secs_f64()
}
