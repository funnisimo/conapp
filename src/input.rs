use crate::app::{AppEvent, KeyDownEvent, KeyUpEvent, VirtualKeyCode};
use std::collections::HashMap;
use std::iter::Filter;

/// Provides information about user input.
/// Possible values for the `key` scancode parameter can be found in unrust/uni-app's `translate_scan_code`
/// [function](https://github.com/unrust/uni-app/blob/41246b070567e3267f128fff41ededf708149d60/src/native_keycode.rs#L160).
/// Warning, there are some slight variations from one OS to another, for example the `Command`, `F13`, `F14`, `F15` keys
/// only exist on Mac.
///
/// State functions like [`InputApi::key`], [`InputApi::mouse_button`] and [`InputApi::mouse_pos`] always work.
/// On another hand, pressed/released event functions should be called only in the update function.
///
pub trait InputApi {
    /// all events that are recorded since the last frame
    fn events(&self) -> &Vec<AppEvent>;

    // keyboard state
    /// return the current status of a key (true if pressed)
    fn key(&self, key: VirtualKeyCode) -> bool;
    /// return true if a key was pressed since last update.
    fn key_pressed(&self, key: VirtualKeyCode) -> bool;
    /// return an iterator over all the keys that were pressed since last update.
    fn keys_pressed(&self) -> Keys;
    /// return true if a key was released since last update.
    fn key_released(&self, key: VirtualKeyCode) -> bool;
    /// return an iterator over all the keys that were released since last update.
    fn keys_released(&self) -> Keys;

    // mouse
    /// return the current status of a mouse button (true if pressed)
    fn mouse_button(&self, num: usize) -> bool;
    /// return true if a mouse button was pressed since last update.
    fn mouse_button_pressed(&self, num: usize) -> bool;
    /// return true if a mouse button was released since last update.
    fn mouse_button_released(&self, num: usize) -> bool;
    /// return the current mouse position on the screen in percent (0.0-1.0, 0.0-1.0)
    /// give this to the console cell_pos method to get the cell the mouse is in
    fn mouse_pct(&self) -> (f32, f32);

    fn mouse_event(&self) -> bool;
    fn key_event(&self) -> bool;

    fn left_clicked(&self) -> bool;

    /// Whether the window close button was clicked
    fn close_requested(&self) -> bool;
}

pub struct AppInput {
    kdown: HashMap<VirtualKeyCode, bool>,
    kpressed: HashMap<VirtualKeyCode, bool>,
    kreleased: HashMap<VirtualKeyCode, bool>,
    mdown: HashMap<usize, bool>,
    mpressed: HashMap<usize, bool>,
    mreleased: HashMap<usize, bool>,
    // text: Vec<String>,
    close_request: bool,
    mpos: (f32, f32),
    screen_size: (f32, f32),
    // con_size: (f32, f32),
    mouse_offset: (f32, f32),
    // last_pressed: Option<KeyDownEvent>,
    events: Vec<AppEvent>,

    mouse_event: bool,
    key_event: bool,
}

impl AppInput {
    pub fn new(
        (screen_width, screen_height): (u32, u32),
        // (con_width, con_height): (u32, u32),
        (x_offset, y_offset): (u32, u32),
    ) -> Self {
        Self {
            kdown: HashMap::new(),
            kpressed: HashMap::new(),
            kreleased: HashMap::new(),
            mdown: HashMap::new(),
            mpressed: HashMap::new(),
            mreleased: HashMap::new(),
            mpos: (0.0, 0.0),
            // text: Vec::new(),
            close_request: false,
            screen_size: (screen_width as f32, screen_height as f32),
            // con_size: (con_width as f32, con_height as f32),
            mouse_offset: (x_offset as f32, y_offset as f32),
            // last_pressed: None,
            events: Vec::new(),
            mouse_event: false,
            key_event: false,
        }
    }
    fn on_key_down(&mut self, key: &KeyDownEvent) {
        if !self.key(key.key_code) {
            self.kpressed.insert(key.key_code, true);
            self.kdown.insert(key.key_code, true);
        }
    }
    fn on_key_up(&mut self, key: &KeyUpEvent) {
        self.kpressed.insert(key.key_code, false);
        self.kdown.insert(key.key_code, false);
        self.kreleased.insert(key.key_code, true);
    }
    fn on_mouse_down(&mut self, button: usize) {
        if !self.mouse_button(button) {
            self.mpressed.insert(button, true);
            self.mdown.insert(button, true);
        }
    }
    fn on_mouse_up(&mut self, button: usize) {
        self.mpressed.insert(button, false);
        self.mdown.insert(button, false);
        self.mreleased.insert(button, true);
    }
    pub fn on_frame(&mut self) {
        self.mpressed.clear();
        self.mreleased.clear();
        self.kreleased.clear();
        self.kpressed.clear();
        self.close_request = false;
        self.events.clear();
        self.mouse_event = false;
        self.key_event = false;
    }
    pub fn on_event(&mut self, event: &AppEvent) {
        self.events.push(event.clone());

        match event {
            AppEvent::KeyDown(ref key) => {
                self.on_key_down(&key);
                self.key_event = true;
            }
            AppEvent::KeyUp(ref key) => {
                self.on_key_up(&key);
                self.key_event = true;
            }
            AppEvent::CharEvent(ch) => {
                match self.events.iter_mut().rfind(|ev| match ev {
                    AppEvent::KeyDown(_) => true,
                    _ => false,
                }) {
                    Some(AppEvent::KeyDown(ev)) => {
                        ev.key = ch.to_string();
                    }
                    _ => {}
                }
                self.key_event = true;
            }
            AppEvent::MousePos(ref pos) => {
                self.mpos = (
                    // (pos.0 as f32 - self.mouse_offset.0) / self.screen_size.0 * self.con_size.0,
                    // (pos.1 as f32 - self.mouse_offset.1) / self.screen_size.1 * self.con_size.1,
                    (pos.0 as f32 - self.mouse_offset.0) / self.screen_size.0,
                    (pos.1 as f32 - self.mouse_offset.1) / self.screen_size.1,
                );
                self.mouse_event = true;
            }
            AppEvent::MouseDown(ref mouse) => {
                self.on_mouse_down(mouse.button);
                self.mouse_event = true;
            }
            AppEvent::MouseUp(ref mouse) => {
                self.on_mouse_up(mouse.button);
                self.mouse_event = true;
            }
            AppEvent::CloseRequested => {
                self.close_request = true;
            }
            _ => (),
        }
    }
    pub(crate) fn resize(
        &mut self,
        (screen_width, screen_height): (u32, u32),
        // (con_width, con_height): (u32, u32),
        (x_offset, y_offset): (u32, u32),
    ) {
        self.screen_size = (screen_width as f32, screen_height as f32);
        // self.con_size = (con_width as f32, con_height as f32);
        self.mouse_offset = (x_offset as f32, y_offset as f32);
    }
}

impl InputApi for AppInput {
    fn events(&self) -> &Vec<AppEvent> {
        &self.events
    }

    fn key(&self, key_code: VirtualKeyCode) -> bool {
        matches!(self.kdown.get(&key_code), Some(&true))
    }
    fn key_pressed(&self, key_code: VirtualKeyCode) -> bool {
        matches!(self.kpressed.get(&key_code), Some(&true))
    }
    fn keys_pressed(&self) -> Keys {
        Keys {
            inner: self.kpressed.iter().filter(|&(_, &v)| v),
        }
    }
    fn key_released(&self, key_code: VirtualKeyCode) -> bool {
        matches!(self.kreleased.get(&key_code), Some(&true))
    }
    fn keys_released(&self) -> Keys {
        Keys {
            inner: self.kreleased.iter().filter(|&(_, &v)| v),
        }
    }
    fn mouse_button(&self, num: usize) -> bool {
        matches!(self.mdown.get(&num), Some(&true))
    }
    fn mouse_button_pressed(&self, num: usize) -> bool {
        matches!(self.mpressed.get(&num), Some(&true))
    }
    fn mouse_button_released(&self, num: usize) -> bool {
        matches!(self.mreleased.get(&num), Some(&true))
    }

    fn mouse_event(&self) -> bool {
        self.mouse_event
    }
    fn key_event(&self) -> bool {
        self.key_event
    }

    fn left_clicked(&self) -> bool {
        self.mouse_button_pressed(0)
    }

    // returns the x,y percent of the mouse on the window - (0.0-1.0, 0.0-1.0)
    fn mouse_pct(&self) -> (f32, f32) {
        self.mpos
    }
    fn close_requested(&self) -> bool {
        self.close_request
    }
}

type KeyMapFilter<'a> = Filter<
    std::collections::hash_map::Iter<'a, VirtualKeyCode, bool>,
    fn(&(&'a VirtualKeyCode, &'a bool)) -> bool,
>;

/// An iterator visiting all keys in arbitrary order.
pub struct Keys<'a> {
    inner: KeyMapFilter<'a>,
}

impl<'a> Iterator for Keys<'a> {
    type Item = &'a VirtualKeyCode;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(c, _)| c)
    }
}

// fn translate_key(ev: &KeyDownEvent) -> Option<String> {
//     let mut out = "".to_owned();
//     if ev.ctrl {
//         out += "^";
//     }
//     if ev.alt {
//         out += "#";
//     }
//     if let Some(key) = match (ev.key.as_str(), ev.shift) {
//         ("Digit1", false) => Some("1"),
//         ("Digit1", true) => Some("!"),
//         ("Digit2", false) => Some("2"),
//         ("Digit2", true) => Some("@"),
//         ("Digit3", false) => Some("3"),
//         ("Digit3", true) => Some("#"),
//         ("Digit4", false) => Some("4"),
//         ("Digit4", true) => Some("$"),
//         ("Digit5", false) => Some("5"),
//         ("Digit5", true) => Some("%"),
//         ("Digit6", false) => Some("6"),
//         ("Digit6", true) => Some("^"),
//         ("Digit7", false) => Some("7"),
//         ("Digit7", true) => Some("&"),
//         ("Digit8", false) => Some("8"),
//         ("Digit8", true) => Some("*"),
//         ("Digit9", false) => Some("9"),
//         ("Digit9", true) => Some("("),
//         ("Digit0", false) => Some("0"),
//         ("Digit0", true) => Some(")"),

//         ("KeyA", _) => Some("a"),
//         ("KeyB", _) => Some("b"),
//         ("KeyC", _) => Some("c"),
//         ("KeyD", _) => Some("d"),
//         ("KeyE", _) => Some("e"),
//         ("KeyF", _) => Some("f"),
//         ("KeyG", _) => Some("g"),
//         ("KeyH", _) => Some("h"),
//         ("KeyI", _) => Some("i"),
//         ("KeyJ", _) => Some("j"),
//         ("KeyK", _) => Some("k"),
//         ("KeyL", _) => Some("l"),
//         ("KeyM", _) => Some("m"),
//         ("KeyN", _) => Some("n"),
//         ("KeyO", _) => Some("o"),
//         ("KeyP", _) => Some("p"),
//         ("KeyQ", _) => Some("q"),
//         ("KeyR", _) => Some("r"),
//         ("KeyS", _) => Some("s"),
//         ("KeyT", _) => Some("t"),
//         ("KeyU", _) => Some("u"),
//         ("KeyV", _) => Some("v"),
//         ("KeyW", _) => Some("w"),
//         ("KeyX", _) => Some("x"),
//         ("KeyY", _) => Some("y"),
//         ("KeyZ", _) => Some("z"),

//         ("Space", _) => Some(" "),
//         ("Numpad0", _) => Some("0"),
//         ("Numpad1", _) => Some("1"),
//         ("Numpad2", _) => Some("2"),
//         ("Numpad3", _) => Some("3"),
//         ("Numpad4", _) => Some("4"),
//         ("Numpad5", _) => Some("5"),
//         ("Numpad6", _) => Some("6"),
//         ("Numpad7", _) => Some("7"),
//         ("Numpad8", _) => Some("8"),
//         ("Numpad9", _) => Some("9"),
//         ("NumpadAdd", _) => Some("+"),
//         ("NumpadDecimal", _) => Some("."),
//         ("NumpadDivide", _) => Some("/"),
//         ("NumpadMultiply", _) => Some("*"),
//         ("NumpadComma", _) => Some(","),
//         ("NumpadEqual", _) => Some("="),
//         ("NumpadSubtract", _) => Some("-"),

//         ("Backquote", false) => Some("`"),
//         ("Backquote", true) => Some("~"),
//         ("Minus", false) => Some("-"),
//         ("Minus", true) => Some("_"),
//         ("Equal", false) => Some("="),
//         ("Equal", true) => Some("+"),

//         ("BracketLeft", false) => Some("["),
//         ("BracketLeft", true) => Some("{"),
//         ("BracketRight", false) => Some("]"),
//         ("BracketRight", true) => Some("}"),
//         ("Backslash", false) => Some("\\"),
//         ("Backslash", true) => Some("|"),

//         ("Semicolon", false) => Some(";"),
//         ("Semicolon", true) => Some(":"),
//         ("Apostrophe", false) => Some("'"),
//         ("Apostrophe", true) => Some("\""),

//         ("Comma", false) => Some(","),
//         ("Comma", true) => Some("<"),
//         ("Period", false) => Some("."),
//         ("Period", true) => Some(">"),
//         ("Slash", false) => Some("/"),
//         ("Slash", true) => Some("?"),

//         (x, _) if x.len() > 0 => Some(x),
//         _ => None,
//     } {
//         if ev.shift {
//             out += key.to_uppercase().as_str();
//         } else {
//             out += key;
//         }
//     }

//     if out.len() > 0 {
//         Some(out)
//     } else {
//         None
//     }
// }
