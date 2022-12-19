use conapp::{codepage437::to_glyph, *};

struct MainScreen {
    con: Console,
    pos: (i32, i32),
}

impl ScreenCreator for MainScreen {
    fn create(app: &mut dyn AppContext) -> Box<dyn Screen> {
        let con = Console::new(80, 50, "resources/terminal_8x8.png", app);
        let pos = (40, 25);
        Box::new(MainScreen { con, pos })
    }
}

impl Screen for MainScreen {
    fn input(&mut self, _app: &mut dyn AppContext, ev: &AppEvent) -> ScreenResult {
        match ev {
            AppEvent::KeyDown(key_down) => match key_down.key_code {
                VirtualKeyCode::Left => self.pos.0 = (self.pos.0 - 1).max(0),
                VirtualKeyCode::Right => self.pos.0 = (self.pos.0 + 1).min(79),
                VirtualKeyCode::Up => self.pos.1 = (self.pos.1 - 1).max(0),
                VirtualKeyCode::Down => self.pos.1 = (self.pos.1 + 1).min(49),
                _ => return ScreenResult::Quit,
            },
            AppEvent::MouseDown(_) => return ScreenResult::Quit,
            _ => {}
        }
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut dyn AppContext) {
        let buffer = self.con.buffer_mut();
        buffer.clear(false, false, true);
        buffer.draw(
            self.pos.0,
            self.pos.1,
            to_glyph('@'),
            RGBA::rgb(255, 255, 0),
            RGBA::rgb(0, 0, 0),
        );
        self.con.render(app.gl());
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Basic Example")
        .font("resources/terminal_8x8.png")
        .build();
    app.run::<MainScreen>();
}
