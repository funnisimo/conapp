mod app;
mod builder;
pub mod codepage437;
mod context;
pub mod draw;
mod file;
mod font;
mod img;
mod input;
mod rgba;
mod runner;
mod screen;
mod simple;

pub use app::{
    App, AppConfig, AppEvent, KeyDownEvent, KeyUpEvent, MouseButtonEvent, VirtualKeyCode,
};
pub use builder::*;
pub use context::*;
pub use draw::{BorderType, TextAlign};
pub use file::*;
pub use font::Font;
pub use img::*;
pub use input::AppInput;
pub use rgba::*;
pub use runner::*;
pub use screen::*;
pub use simple::{subcell_console, Buffer, Console, Glyph};

pub fn console<T: AsRef<str>>(msg: T) {
    app::App::print(msg.as_ref());
}
