use conapp::*;

// This is similar to doryen-rs/examples/basic

// this part makes it possible to compile to wasm32 target
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    main();
    Ok(())
}

/*
Apart from the basic real-time walking, this example shows how screenshots can be captured in-game.
Because it uses UpdateEvent, any combination of keys can be specified to activate it.
*/

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;

struct MyRoguelike {
    player_pos: (i32, i32),
    mouse_pos: (f32, f32),
    screenshot_idx: usize,
}

impl Engine for MyRoguelike {
    fn init(&mut self, _api: &mut dyn AppContext) {
        register_color("white", (255, 255, 255, 255));
        register_color("red", (255, 92, 92, 255));
        register_color("blue", (192, 192, 255, 255));
    }

    fn input(&mut self, api: &mut dyn AppContext) -> Option<RunnerEvent> {
        let input = api.input();

        for ev in input.events() {
            match ev {
                AppEvent::KeyDown(ev) => match ev.code.as_str() {
                    "ArrowLeft" => {
                        self.player_pos.0 = (self.player_pos.0 - 1).max(1);
                    }
                    "ArrowRight" => {
                        self.player_pos.0 = (self.player_pos.0 + 1).min(CONSOLE_WIDTH as i32 - 2);
                    }
                    "ArrowUp" => {
                        self.player_pos.1 = (self.player_pos.1 - 1).max(1);
                    }
                    "ArrowDown" => {
                        self.player_pos.1 = (self.player_pos.1 + 1).min(CONSOLE_HEIGHT as i32 - 2);
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        self.mouse_pos = input.mouse_pct();

        // capture the screen
        if input.key(VirtualKeyCode::LControl) && input.key_pressed(VirtualKeyCode::S) {
            self.screenshot_idx += 1;
            return Some(RunnerEvent::Capture(format!(
                "screenshot_{:03}.png",
                self.screenshot_idx
            )));
        }

        None
    }

    fn render(&mut self, api: &mut dyn AppContext) {
        let con = api.get_console_mut(0);
        let mouse_pos = con.cell_pos(self.mouse_pos);

        let buffer = con.buffer_mut();
        buffer.rectangle(
            0,
            0,
            CONSOLE_WIDTH,
            CONSOLE_HEIGHT,
            Some((128, 128, 128, 255)),
            Some((0, 0, 0, 255)),
            Some('.' as u32),
        );
        buffer.area(
            10,
            10,
            5,
            5,
            Some((255, 64, 64, 255)),
            Some((128, 32, 32, 255)),
            Some('&' as u32),
        );
        buffer.glyph(self.player_pos.0, self.player_pos.1, '@' as u32);
        buffer.fore(self.player_pos.0, self.player_pos.1, (255, 255, 255, 255));
        buffer.print_color(
            (CONSOLE_WIDTH / 2) as i32,
            (CONSOLE_HEIGHT - 1) as i32,
            "#[red]arrows#[white] : move - #[red]CTRL-S#[white] : save screenshot",
            TextAlign::Center,
            None,
        );
        if let Some(mouse_pos) = mouse_pos {
            buffer.print_color(
                (CONSOLE_WIDTH / 2) as i32,
                (CONSOLE_HEIGHT - 3) as i32,
                &format!(
                    "#[white]Mouse coordinates: #[red]{}, {}",
                    mouse_pos.0, mouse_pos.1
                ),
                TextAlign::Center,
                None,
            );
            buffer.back(mouse_pos.0 as i32, mouse_pos.1 as i32, (255, 255, 255, 255));
        }
        buffer.print_color(
            5,
            5,
            "#[blue]This blue text contains a #[red]red#[] word",
            TextAlign::Left,
            None,
        );
    }
}

impl MyRoguelike {
    pub fn new() -> Self {
        Self {
            player_pos: ((CONSOLE_WIDTH / 2) as i32, (CONSOLE_HEIGHT / 2) as i32),
            mouse_pos: (0.0, 0.0),
            screenshot_idx: 0,
        }
    }
}

fn main() {
    let mut app = App::new(AppOptions {
        console_width: CONSOLE_WIDTH,
        console_height: CONSOLE_HEIGHT,
        screen_width: CONSOLE_WIDTH * 8,
        screen_height: CONSOLE_HEIGHT * 8,
        window_title: "my roguelike".to_owned(),
        // font_path: "brogue_32x58.png".to_owned(),
        ..Default::default()
    });
    app.set_engine(Box::new(MyRoguelike::new()));
    app.run();
}
