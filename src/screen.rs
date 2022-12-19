use crate::AppEvent;

use super::AppContext;
use std::fmt::Debug;

pub enum ScreenResult {
    Continue,
    Push(Box<dyn Screen>),
    Replace(Box<dyn Screen>),
    Pop,
    Quit,
}

impl Debug for ScreenResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ScreenResult::Continue => "Continue",
                ScreenResult::Push(_) => "Push",
                ScreenResult::Replace(_) => "Replace",
                ScreenResult::Pop => "Pop",
                ScreenResult::Quit => "Quit",
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
            (_, _) => false,
        }
    }
}

pub trait ScreenCreator {
    fn create(ctx: &mut dyn AppContext) -> Box<dyn Screen>;
}

#[allow(unused_variables)]
pub trait Screen {
    fn setup(&mut self, ctx: &mut dyn AppContext) {}
    fn resize(&mut self, ctx: &mut dyn AppContext) {}

    fn is_full_screen(&self) -> bool {
        true
    }
    fn needs_background_update(&self) -> bool {
        false
    }

    fn pause(&mut self, ctx: &mut dyn AppContext) {}
    fn resume(&mut self, ctx: &mut dyn AppContext) {}

    fn input(&mut self, ctx: &mut dyn AppContext, event: &AppEvent) -> ScreenResult {
        ScreenResult::Continue
    }

    fn update(&mut self, ctx: &mut dyn AppContext, frame_time_ms: f32) -> ScreenResult {
        ScreenResult::Continue
    }

    fn render(&mut self, ctx: &mut dyn AppContext) {}

    fn teardown(&mut self, ctx: &mut dyn AppContext) {}
}
