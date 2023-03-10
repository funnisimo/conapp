use conapp::*;

const FONT: &str = "resources/terminal_8x8.png";

const WHITE: RGBA = RGBA::rgb(255, 255, 255);
const _RED: RGBA = RGBA::rgb(255, 0, 0);
const GREEN: RGBA = RGBA::rgb(0, 255, 0);
const _BLUE: RGBA = RGBA::rgb(0, 0, 255);
const BLACK: RGBA = RGBA::rgb(0, 0, 0);
const GRAY: RGBA = RGBA::rgb(128, 128, 128);

fn my_to_rgba(name: &str) -> Option<RGBA> {
    match name {
        "white" => Some(WHITE),
        "red" => Some(RGBA::rgb(255, 92, 92)),
        "green" => Some(GREEN),
        "blue" => Some(RGBA::rgb(192, 192, 255)),
        "black" => Some(BLACK),
        "gray" => Some(GRAY),
        _ => to_rgba(name),
    }
}

struct ColoredScreen {
    con: Console,
}

impl ColoredScreen {
    fn new() -> Box<Self> {
        let con = Console::new(80, 50, FONT);
        Box::new(ColoredScreen { con })
    }
}

impl Screen for ColoredScreen {
    fn setup(&mut self, _app: &mut AppContext) {
        let mut buffer = self.con.buffer_mut();
        buffer.clear(true, false, false);

        // PLAIN (NO BG, NO WIDTH)
        let y = 2;

        let mut draw = draw::colored(&mut buffer)
            .to_rgba(&my_to_rgba)
            .fg(RGBA::rgb(192, 32, 32));
        draw.print(2, y, "No #[0F0]bg#[], no #[00f]width#[]");
        draw.print_lines(
            2,
            y + 2,
            "print_lines can\nhandle #[32,32,220]newlines#[], but\nwill not #[blue]word wrap#[].",
        );
        draw.wrap(26, y,  "Inside a #[396]call to wrap#[], you can place a #[ee3]long text#[] and it will automatically be #[66f]wrapped#[] at the width you specify.  Or at the #[dd3]end of the buffer#[].");

        let y = 10;

        let mut draw = draw::colored(&mut buffer)
            .to_rgba(&my_to_rgba)
            .fg(RGBA::rgb(255, 0, 255))
            .bg(RGBA::rgb(0, 64, 255));
        draw.print(2, y, "No #[0F0]bg#[], no #[00f]width#[]");
        draw.print_lines(
            2,
            y + 2,
            "print_lines can\nhandle #[32,32,220]newlines#[], but\nwill not #[blue]word wrap#[].",
        );
        draw.wrap(26, y,  "Inside a #[396]call to wrap#[], you can place a #[ee3]long text#[] and it will automatically be #[66f]wrapped#[] at the width you specify.  Or at the #[dd3]end of the buffer#[].");

        let y = 18;

        draw::plain(buffer).print(26, y, "Align::Left");
        draw::plain(buffer)
            .align(TextAlign::Center)
            .print(52, y, "Align::Center");
        draw::plain(buffer)
            .align(TextAlign::Right)
            .print(78, y, "Align::Right");

        // width, no bg
        let y = 20;

        let mut draw = draw::colored(&mut buffer)
            .to_rgba(&my_to_rgba)
            .fg(RGBA::rgb(64, 128, 32))
            .width(15);
        draw.print(2, y, "No #[0F0]bg#[], no #[00f]width#[]");
        draw.print_lines(
            2,
            y + 2,
            "print_lines can\nhandle #[32,32,220]newlines#[], but\nwill not #[blue]word wrap#[].",
        );
        draw.wrap(26, y,  "Inside a #[396]call to wrap#[], you can place a #[ee3]long text#[] and it will automatically be #[66f]wrapped#[] at the width you specify.  Or at the #[dd3]end of the buffer#[].");
        draw = draw.align(TextAlign::Center);
        draw.wrap(52, y,  "Inside a #[396]call to wrap#[], you can place a #[ee3]long text#[] and it will automatically be #[66f]wrapped#[] at the width you specify.  Or at the #[dd3]end of the buffer#[].");
        draw = draw.align(TextAlign::Right);
        draw.wrap(78, y,  "Inside a #[396]call to wrap#[], you can place a #[ee3]long text#[] and it will automatically be #[66f]wrapped#[] at the width you specify.  Or at the #[dd3]end of the buffer#[].");

        // width, no bg
        let y = 33;

        let mut draw = draw::colored(&mut buffer)
            .to_rgba(&my_to_rgba)
            .fg(RGBA::rgb(255, 255, 255))
            .bg(RGBA::rgb(0, 64, 255))
            .width(15);
        draw.print(2, y, "No #[0F0]bg#[], no #[00f]width#[]");
        draw.print_lines(
            2,
            y + 2,
            "print_lines can\nhandle #[32,32,220]newlines#[], but\nwill not #[blue]word wrap#[].",
        );
        draw.wrap(26, y,  "Inside a #[396]call to wrap#[], you can place a #[ee3]long text#[] and it will automatically be #[66f]wrapped#[] at the width you specify.  Or at the #[dd3]end of the buffer#[].");
        draw = draw.align(TextAlign::Center);
        draw.wrap(52, y,  "Inside a #[396]call to wrap#[], you can place a #[ee3]long text#[] and it will automatically be #[66f]wrapped#[] at the width you specify.  Or at the #[dd3]end of the buffer#[].");
        draw = draw.align(TextAlign::Right);
        draw.wrap(78, y,  "Inside a #[396]call to wrap#[], you can place a #[ee3]long text#[] and it will automatically be #[66f]wrapped#[] at the width you specify.  Or at the #[dd3]end of the buffer#[].");
    }

    fn render(&mut self, app: &mut AppContext) {
        self.con.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Basic Example")
        .font(FONT)
        .build();
    app.run_screen(ColoredScreen::new());
}
