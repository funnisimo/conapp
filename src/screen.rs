use super::AppContext;
use crate::AppEvent;
use std::fmt::Debug;

/// The result of an evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum MsgData {
    Number(i32),
    Float(f32),
    Text(String),
    Boolean(bool),
    List(Vec<MsgData>),
    Error,
}

impl MsgData {
    pub fn as_float(&self) -> Result<f32, ()> {
        match self {
            MsgData::Number(v) => Ok(*v as f32),
            MsgData::Float(v) => Ok(*v),
            MsgData::Text(v) => match v.parse::<f32>() {
                Err(_) => Err(()),
                Ok(v) => Ok(v),
            },
            MsgData::Boolean(v) => match v {
                true => Ok(1.0),
                false => Ok(0.0),
            },
            // Value::Blank => Ok(0.0),
            _ => Err(()),
        }
    }

    pub fn as_string(&self) -> Result<String, ()> {
        match self {
            MsgData::Number(v) => Ok(format!("{}", v)),
            MsgData::Float(v) => Ok(format!("{}", v)),
            MsgData::Text(v) => Ok(v.clone()),
            MsgData::Boolean(v) => match v {
                true => Ok("1.0".to_owned()),
                false => Ok("0.0".to_owned()),
            },
            // Value::Blank => Ok("".to_owned()),
            _ => Err(()),
        }
    }

    pub fn as_bool(&self) -> Result<bool, ()> {
        match self {
            MsgData::Number(v) => Ok(*v != 0),
            MsgData::Float(v) => Ok(*v != 0.0),
            MsgData::Text(v) => Ok(v.len() > 0),
            MsgData::Boolean(v) => match v {
                true => Ok(true),
                false => Ok(false),
            },
            // Value::Blank => Ok(false),
            _ => Err(()),
        }
    }
}

impl From<i32> for MsgData {
    fn from(v: i32) -> Self {
        MsgData::Number(v)
    }
}

impl From<f32> for MsgData {
    fn from(v: f32) -> Self {
        MsgData::Float(v)
    }
}

impl From<&str> for MsgData {
    fn from(v: &str) -> Self {
        MsgData::Text(v.to_owned())
    }
}

impl From<bool> for MsgData {
    fn from(v: bool) -> Self {
        MsgData::Boolean(v)
    }
}

/// The result from a call to one of the [`Screen`] event functions
pub enum ScreenResult {
    /// Continue to process the frame
    Continue,
    /// Push a new screen onto the stack
    Push(Box<dyn Screen>),
    /// Replace the current screen with this new one
    Replace(Box<dyn Screen>),
    /// Pop the current screen off the stack
    Pop,
    /// Quit the application
    Quit,
    /// Save a screen capture to the given filename
    Capture(String),
}

impl Debug for ScreenResult {
    /// Shows the name of the enum value
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ScreenResult::Continue => "Continue".to_owned(),
                ScreenResult::Push(_) => "Push".to_owned(),
                ScreenResult::Replace(_) => "Replace".to_owned(),
                ScreenResult::Pop => "Pop".to_owned(),
                ScreenResult::Quit => "Quit".to_owned(),
                ScreenResult::Capture(name) => format!("Capture({})", name),
            }
        )
    }
}

impl PartialEq for ScreenResult {
    /// Compares the enum value, but not any Screen
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ScreenResult::Continue, ScreenResult::Continue) => true,
            (ScreenResult::Push(_), ScreenResult::Push(_)) => true,
            (ScreenResult::Replace(_), ScreenResult::Replace(_)) => true,
            (ScreenResult::Pop, ScreenResult::Pop) => true,
            (ScreenResult::Quit, ScreenResult::Quit) => true,
            (ScreenResult::Capture(a), ScreenResult::Capture(b)) => a == b,
            (_, _) => false,
        }
    }
}

/// A screen to handle input, update, and render events
#[allow(unused_variables)]
pub trait Screen {
    /// Called once, when the screen is first added to the [`crate::Runner`]
    fn setup(&mut self, app: &mut AppContext) {
        self.resize(app);
    }

    /// Called when the app is resized
    fn resize(&mut self, app: &mut AppContext) {}

    /// Returns whether or not this screen is full size, if not the [`crate::Runner`] will render the screens below.
    fn is_full_screen(&self) -> bool {
        true
    }

    /// Returns whether or not this screen should get update calls when it is not on top
    fn needs_background_update(&self) -> bool {
        false
    }

    /// Called when another screen is pushed on top of this one
    fn pause(&mut self, app: &mut AppContext) {}

    /// Called when this screen becomes the topmost screen
    fn resume(&mut self, app: &mut AppContext) {}

    /// called when a message is sent via app.send_message(...)
    fn message(
        &mut self,
        app: &mut AppContext,
        id: String,
        value: Option<MsgData>,
    ) -> ScreenResult {
        ScreenResult::Continue
    }

    /// Called once for each input event that occurred in this frame
    fn input(&mut self, app: &mut AppContext, event: &AppEvent) -> ScreenResult {
        ScreenResult::Continue
    }

    /// Called at the goal fps, can be called multiple times per frame
    fn update(&mut self, app: &mut AppContext, frame_time_ms: f64) -> ScreenResult {
        ScreenResult::Continue
    }

    /// Called once at the end of the frame
    fn render(&mut self, app: &mut AppContext) {}

    /// Called when this screen is popped from the stack
    fn teardown(&mut self, app: &mut AppContext) {}
}
