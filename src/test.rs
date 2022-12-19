// #![allow(unused_variables)]
// use uni_gl::WebGLRenderingContext;

// use super::input::AppInput;
// use super::*;
// // use crate::rgba::RGBA;
// // use crate::Glyph;
// // use crate::world::{global_world, global_world_mut, World};

// pub struct TestAppContext {
//     size: (u32, u32),
//     input: AppInput,
// }

// impl TestAppContext {
//     pub fn new(width: u32, height: u32) -> Self {
//         TestAppContext {
//             size: (width, height),
//             input: AppInput::new((width, height), (0, 0)),
//         }
//     }
// }

// // impl DrawTarget for TestAppContext {
// //     fn size(&self) -> (u32, u32) {
// //         self.size
// //     }

// //     fn draw(&mut self, x: i32, y: i32, glyph: Option<Glyph>, fg: Option<RGBA>, bg: Option<RGBA>) {}

// //     fn draw_bg(&mut self, x: i32, y: i32, bg: RGBA) {}
// // }

// impl AppContext for TestAppContext {
//     fn input(&self) -> &dyn InputApi {
//         &self.input
//     }

//     fn gl(&self) -> &WebGLRenderingContext {}

//     // fn get_console(&self, idx: usize) -> Option<&Console> {
//     //     None
//     // }

//     // fn get_console_mut(&mut self, idx: usize) -> Option<&mut Console> {
//     //     None
//     // }

//     // fn mouse_point(&self, console_id: usize) -> Option<Point> {
//     //     Some(Point::zero())
//     // }
//     // fn has_mouse_event(&self) -> bool {
//     //     false
//     // }
//     // fn left_clicked(&self) -> bool {
//     //     false
//     // }

//     // fn has_key_event(&self) -> bool {
//     //     false
//     // }

//     fn get_screen_size(&self) -> (u32, u32) {
//         self.size
//     }
//     fn fps(&self) -> u32 {
//         60
//     }
//     fn average_fps(&self) -> u32 {
//         60
//     }
//     fn frame_time_ms(&self) -> f32 {
//         16.0
//     }

//     // fn clear_all(&mut self) -> () {}

//     // fn draw_target(&mut self, index: usize) -> Option<&mut dyn DrawTarget> {
//     //     Some(self)
//     // }
// }

// pub fn test_app_context(width: u32, height: u32) -> TestAppContext {
//     TestAppContext::new(width, height)
// }
