use conapp::*;
use std::cell::RefCell;
use std::rc::Rc;

// doryen-rs/examples/subcell

// This example does NOT do subcell rendering
// subcell rendering is not supported by codepate437 natively
// you must implement a font that has subcell chars yourself

const FONT: &str = "resources/terminal_8x8.png";

struct MyRoguelike {
    con: Console,
    subcell: Console,
    skull: Rc<RefCell<Image>>,
}

impl ScreenCreator for MyRoguelike {
    fn create(app: &mut AppContext) -> Box<dyn Screen> {
        let font = app.load_font(FONT);
        let con = Console::new(60, 80, font);

        Box::new(MyRoguelike {
            con,
            subcell: subcell_console(30, 40, app).extents(0.25, 0.25, 0.75, 0.75),
            skull: app.load_image("resources/skull.png").unwrap(),
        })
    }
}

impl Screen for MyRoguelike {
    fn render(&mut self, app: &mut AppContext) {
        // text
        let buffer = self.con.buffer_mut();
        buffer.fill(
            Some('.' as u32),
            Some(RGBA::rgba(255, 0, 255, 255)),
            Some(RGBA::rgb(32, 32, 32)),
        );
        draw::plain(buffer)
            .align(TextAlign::Center)
            .fg(RGBA::rgb(0, 128, 255))
            .print_lines(30, 10, "This is a 60x80 png\non a 30x40 console.");

        self.con.render(app.gl());

        // image
        self.subcell.buffer_mut().clear(true, true, true);
        draw::subcell(self.subcell.buffer_mut())
            .transparent(RGBA::rgba(0, 0, 0, 255))
            .blit(&*self.skull.borrow(), 0, 0, 0, 0, None, None);
        self.subcell.render(app.gl());
    }
}

fn main() {
    let app = AppBuilder::new(768, 1024)
        .title("SubCell Resolution Example")
        .font(FONT)
        .build();
    app.run::<MyRoguelike>();
}
