use super::AppContext;
use crate::AppEvent;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum DataConvertError {
    WrongType,
    Negative,
}

/// The result of an evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum MsgData {
    Index(usize),
    Number(i32),
    Float(f32),
    Text(String),
    Boolean(bool),
    List(Vec<MsgData>),
    Error,
}

impl TryInto<usize> for MsgData {
    type Error = DataConvertError;

    fn try_into(self) -> Result<usize, DataConvertError> {
        match self {
            MsgData::Index(v) => Ok(v),
            MsgData::Number(v) => Ok(v as usize),
            MsgData::Float(v) => Ok(v.floor() as usize),
            MsgData::Text(v) => match v.parse::<usize>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            MsgData::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },
            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<i32> for MsgData {
    type Error = DataConvertError;

    fn try_into(self) -> Result<i32, DataConvertError> {
        match self {
            MsgData::Index(v) => Ok(v as i32),
            MsgData::Number(v) => Ok(v),
            MsgData::Float(v) => Ok(v.floor() as i32),
            MsgData::Text(v) => match v.parse::<i32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            MsgData::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },
            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<u32> for MsgData {
    type Error = DataConvertError;

    fn try_into(self) -> Result<u32, DataConvertError> {
        match self {
            MsgData::Index(v) => Ok(v as u32),
            MsgData::Number(v) => match v >= 0 {
                true => Ok(v as u32),
                false => Err(DataConvertError::Negative),
            },
            MsgData::Float(v) => match v >= 0.0 {
                true => Ok(v.floor() as u32),
                false => Err(DataConvertError::Negative),
            },
            MsgData::Text(v) => match v.parse::<u32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            MsgData::Boolean(v) => match v {
                true => Ok(1),
                false => Ok(0),
            },
            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<f32> for MsgData {
    type Error = DataConvertError;

    fn try_into(self) -> Result<f32, DataConvertError> {
        match self {
            MsgData::Index(v) => Ok(v as f32),
            MsgData::Number(v) => Ok(v as f32),
            MsgData::Float(v) => Ok(v),
            MsgData::Text(v) => match v.parse::<f32>() {
                Err(_) => Err(DataConvertError::WrongType),
                Ok(v) => Ok(v),
            },
            MsgData::Boolean(v) => match v {
                true => Ok(1.0),
                false => Ok(0.0),
            },
            // Value::Blank => Ok(0.0),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl TryInto<bool> for MsgData {
    type Error = DataConvertError;

    fn try_into(self) -> Result<bool, DataConvertError> {
        match self {
            MsgData::Index(v) => Ok(v != 0),
            MsgData::Number(v) => Ok(v != 0),
            MsgData::Float(v) => Ok(v != 0.0),
            MsgData::Text(v) => Ok(v.len() > 0),
            MsgData::Boolean(v) => match v {
                true => Ok(true),
                false => Ok(false),
            },
            // Value::Blank => Ok(false),
            _ => Err(DataConvertError::WrongType),
        }
    }
}

impl Display for MsgData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MsgData::Index(v) => write!(f, "{}", v),
            MsgData::Number(v) => write!(f, "{}", v),
            MsgData::Float(v) => write!(f, "{}", v),
            MsgData::Text(v) => write!(f, "{}", v),
            MsgData::Boolean(v) => match v {
                true => write!(f, "true"),
                false => write!(f, "false"),
            },
            MsgData::List(data) => {
                write!(f, "[")?;
                for (i, item) in data.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            // Value::Blank => Ok("".to_owned()),
            MsgData::Error => write!(f, "!ERROR!"),
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

impl From<String> for MsgData {
    fn from(v: String) -> Self {
        MsgData::Text(v)
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
    fn message(&mut self, app: &mut AppContext, id: String, data: Option<MsgData>) -> ScreenResult {
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
