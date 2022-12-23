use conapp::*;

const FONT: &str = "resources/terminal_8x8.png";

struct TextScreen {
    con: Console,
}

impl ScreenCreator for TextScreen {
    fn create(app: &mut dyn AppContext) -> Box<dyn Screen> {
        let font = app.load_font(FONT);
        let con = Console::new(80, 50, font);

        Box::new(TextScreen { con })
    }
}

impl Screen for TextScreen {
    fn setup(&mut self, _app: &mut dyn AppContext) {
        let buffer = self.con.buffer_mut();
        buffer.clear(true, false, false);

        // PLAIN (NO BG, NO WIDTH)
        let y = 2;

        let mut draw = draw::plain(buffer).fg(RGBA::rgb(192, 32, 32));
        draw.print(5, y, "No bg, no width");
        draw.print_lines(
            5,
            y + 2,
            "print_lines can\nhandle newlines, but\nwill not word wrap.",
        );
        draw.wrap(30, y,  "Inside a call to wrap, you can place a long text and it will automatically be wrapped at the width you specify.  Or at the end of the buffer.");

        let y = 10;

        let mut draw = draw::plain(buffer)
            .fg(RGBA::rgb(255, 0, 255))
            .bg(RGBA::rgb(0, 64, 255));
        draw.print(5, y, "width=15, but no bg");
        draw.print_lines(
            5,
            y + 2,
            "print_lines can\nhandle newlines, but\nwill not word wrap.",
        );
        draw.wrap(30, y,  "Inside a call to wrap, you can place a long text and it will automatically be wrapped at the width you specify.  Or at the end of the buffer.");

        // width, no bg
        let y = 18;

        let mut draw = draw::plain(buffer).fg(RGBA::rgb(64, 128, 32)).width(15);
        draw.print(5, y, "With bg, no width");
        draw.print_lines(
            5,
            y + 2,
            "print_lines can\nhandle newlines, but\nwill not word wrap.",
        );
        draw.wrap(30, y,  "Inside a call to wrap, you can place a long text and it will automatically be wrapped at the width you specify.  Or at the end of the buffer.");

        // width, no bg
        let y = 31;

        let mut draw = draw::plain(buffer)
            .fg(RGBA::rgb(255, 255, 255))
            .bg(RGBA::rgb(0, 64, 255))
            .width(15);
        draw.print(5, y, "width=15 and bg=blueish");
        draw.print_lines(
            5,
            y + 2,
            "print_lines can\nhandle newlines, but\nwill not word wrap.",
        );
        draw.wrap(30, y,  "Inside a call to wrap, you can place a long text and it will automatically be wrapped at the width you specify.  Or at the end of the buffer.");

        draw::plain(buffer).print(63, 20, "TextAlign::Left");
        draw::plain(buffer)
            .align(TextAlign::Center)
            .print(63, 22, "TextAlign::Center");
        draw::plain(buffer)
            .align(TextAlign::Right)
            .print(63, 24, "TextAlign::Right");
    }

    fn render(&mut self, app: &mut dyn AppContext) {
        self.con.render(app.gl());
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Basic Example")
        .font(FONT)
        .build();
    app.run::<TextScreen>();
}
