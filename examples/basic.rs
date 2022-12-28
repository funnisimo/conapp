use conapp::*;

const FONT: &str = "resources/terminal_8x8.png";

struct MainScreen {
    con: Console,
    pos: (i32, i32),
}

impl ScreenCreator for MainScreen {
    fn create(app: &mut dyn AppContext) -> Box<dyn Screen> {
        let font = app.get_font(FONT);
        let con = Console::new(80, 50, font);
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
        buffer.clear(true, false, false);
        buffer.draw(
            self.pos.0,
            self.pos.1,
            '@' as Glyph,
            RGBA::rgb(255, 255, 0),
            RGBA::rgb(0, 0, 0),
        );

        draw::plain(buffer)
            .fg(RGBA::rgb(255, 0, 255))
            .print(10, 10, "Hello World");

        draw::colored(buffer).fg(RGBA::rgb(255, 255, 255)).print(
            10,
            12,
            "Use the #[#F00]arrow keys#[] to move the '#[#FF0]@#[]' symbol around.",
        );

        draw::colored(buffer).fg(RGBA::rgb(255, 255, 255)).print(
            10,
            14,
            "Click the #[#F00]left mouse button#[] to #[#0FF]Quit#[] the app.",
        );

        self.con.render(app.gl());
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Basic Example")
        .font(FONT)
        .build();
    app.run::<MainScreen>();
}
