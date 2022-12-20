use crate::AppEvent;

use super::AppContext;
use std::fmt::Debug;

pub enum ScreenResult {
    Continue,
    Push(Box<dyn Screen>),
    Replace(Box<dyn Screen>),
    Pop,
    Quit,
    Capture(String), // Take a screenshot to this filename
}

impl Debug for ScreenResult {
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

pub trait ScreenCreator {
    fn create(ctx: &mut dyn AppContext) -> Box<dyn Screen>;
}

#[allow(unused_variables)]
pub trait Screen {
    fn setup(&mut self, app: &mut dyn AppContext) {}
    fn resize(&mut self, app: &mut dyn AppContext) {}

    fn is_full_screen(&self) -> bool {
        true
    }
    fn needs_background_update(&self) -> bool {
        false
    }
    fn ready(&self) -> bool {
        true
    }

    fn pause(&mut self, app: &mut dyn AppContext) {}
    fn resume(&mut self, app: &mut dyn AppContext) {}

    fn input(&mut self, app: &mut dyn AppContext, event: &AppEvent) -> ScreenResult {
        ScreenResult::Continue
    }

    fn update(&mut self, app: &mut dyn AppContext, frame_time_ms: f32) -> ScreenResult {
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut dyn AppContext) {}

    fn teardown(&mut self, app: &mut dyn AppContext) {}
}
