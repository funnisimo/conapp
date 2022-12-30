mod app;
mod buffer;
mod builder;
pub mod codepage437;
mod console;
mod context;
pub mod draw;
mod file;
mod font;
mod img;
mod input;
mod rgba;
mod runner;
mod screen;

pub use app::{
    App, AppConfig, AppEvent, KeyDownEvent, KeyUpEvent, MouseButtonEvent, VirtualKeyCode,
};
pub use buffer::*;
pub use builder::*;
pub use console::*;
pub use context::*;
pub use draw::{BorderType, TextAlign};
pub use file::*;
pub use font::Font;
pub use img::*;
pub use input::AppInput;
pub use rgba::*;
pub use runner::*;
pub use screen::*;

pub fn console<T: AsRef<str>>(msg: T) {
    app::App::print(msg.as_ref());
}
