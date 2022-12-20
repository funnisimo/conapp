use conapp::*;

const FONT: &str = "resources/terminal_8x8.png";

const BLACK: RGBA = RGBA::rgb(0, 0, 0);
const GRAY: RGBA = RGBA::rgb(128, 128, 128);
const RED: RGBA = RGBA::rgb(192, 32, 32);
const YELLOW: RGBA = RGBA::rgb(192, 192, 32);

struct HelloWorld {
    con: Console,
    has_popup: bool,
}
impl ScreenCreator for HelloWorld {
    fn create(app: &mut dyn AppContext) -> Box<dyn Screen> {
        let font = app.get_font(FONT).expect(&format!(
            "Trying to use font that was not loaded.  Add this font to the AppBuilder - {}",
            FONT
        ));

        let con = Console::new(80, 50, font);

        Box::new(HelloWorld {
            con,
            has_popup: false,
        })
    }
}

impl Screen for HelloWorld {
    fn pause(&mut self, _ctx: &mut dyn AppContext) {
        console("pause");
        self.has_popup = true;
    }

    fn resume(&mut self, _ctx: &mut dyn AppContext) {
        console("resume");
        self.has_popup = false;
    }

    fn input(&mut self, app: &mut dyn AppContext, ev: &AppEvent) -> ScreenResult {
        match ev {
            AppEvent::KeyDown(key_down) => {
                let is_full = match key_down.key_code {
                    VirtualKeyCode::F => true,
                    VirtualKeyCode::D => false,
                    _ => {
                        return ScreenResult::Continue;
                    }
                };

                let popup = Box::new(Popup::new(app, is_full, 5000.0));
                return ScreenResult::Push(popup);
            }
            _ => {}
        }

        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut dyn AppContext) {
        let buf = self.con.buffer_mut();
        buf.clear(true, true, true);

        buf.fill(Some('.' as u32), Some(YELLOW), Some(BLACK));

        draw::plain(buf).print(1, 1, "Hello Rust World");
        draw::plain(buf).print(1, 2, "Press D to show a dialog on top of this screen");
        draw::plain(buf).print(1, 3, "Press F to show a full screen dialog");

        if self.has_popup {
            draw::plain(buf).print(1, 5, "Has Popup.");
        }

        self.con.render(app.gl());
    }
}

struct Popup {
    con: Console,
    is_full: bool,
    time_left: f32,
}

impl Popup {
    pub fn new(app: &mut dyn AppContext, is_full: bool, time_left: f32) -> Popup {
        let font = app.get_font(FONT).expect(&format!(
            "Trying to use font that was not loaded.  Add this font to the AppBuilder - {}",
            FONT
        ));

        let con = match is_full {
            true => Console::new(80, 50, font),
            false => Console::new(20, 20, font).extents(0.25, 0.25, 0.5, 0.75),
        };

        Popup {
            con,
            is_full,
            time_left,
        }
    }
}

impl Screen for Popup {
    fn is_full_screen(&self) -> bool {
        self.is_full
    }

    fn input(&mut self, _ctx: &mut dyn AppContext, ev: &AppEvent) -> ScreenResult {
        match ev {
            AppEvent::MouseDown(_) => ScreenResult::Pop,
            _ => ScreenResult::Continue,
        }
    }

    fn update(&mut self, _ctx: &mut dyn AppContext, dt: f32) -> ScreenResult {
        self.time_left -= dt;
        if self.time_left <= 0.0 {
            ScreenResult::Pop
        } else {
            ScreenResult::Continue
        }
    }

    fn render(&mut self, app: &mut dyn AppContext) {
        let screen_pct = app.input().mouse_pct();
        let cell_pct = self.con.cell_pos(screen_pct);

        let buf = self.con.buffer_mut();

        buf.fill(Some('.' as u32), Some(GRAY), Some(BLACK));

        let x = (buf.get_width() as i32 - 20) / 2;
        let y = (buf.get_height() as i32 - 20) / 2;

        draw::frame(buf)
            .border(BorderType::Double)
            .fg(RED)
            .bg(GRAY)
            .fill(Some(' ' as u32), None, Some(BLACK))
            .draw(x, y, 20, 20);

        let t = self.time_left as f32 / 1000.0;
        draw::plain(buf).print(x + 2, y + 3, &format!("Time: {:.1}", t));

        match self.is_full {
            false => {
                draw::plain(buf).print(x + 2, y + 5, "Popup!");
            }
            true => {
                draw::plain(buf).print(x + 2, y + 5, "Full Screen!");
            }
        }

        match cell_pct {
            None => {
                draw::plain(buf).print(x + 2, y + 7, "Mouse: OUT");
            }
            Some(pos) => {
                draw::plain(buf).print(x + 2, y + 7, &format!("Mouse: {:.1},{:.1}", pos.0, pos.1));
            }
        }

        self.con.render(app.gl());
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Popup Example")
        .font(FONT)
        .build();
    app.run::<HelloWorld>();
}