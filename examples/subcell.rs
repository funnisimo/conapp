use conapp::*;

// doryen-rs/examples/subcell

// This example does NOT do subcell rendering
// subcell rendering is not supported by codepate437 natively
// you must implement a font that has subcell chars yourself

const FONT: &str = "resources/terminal_8x8.png";

struct MyRoguelike {
    con: Console,
    skull: Image,
}

impl ScreenCreator for MyRoguelike {
    fn create(app: &mut dyn AppContext) -> Box<dyn Screen> {
        let font = app.get_font(FONT).expect(&format!(
            "Trying to use font that was not loaded.  Add this font to the AppBuilder - {}",
            FONT
        ));

        let con = Console::new(60, 80, font);

        Box::new(MyRoguelike {
            con,
            skull: Image::new("resources/skull.png"),
        })
    }
}

impl Screen for MyRoguelike {
    fn render(&mut self, app: &mut dyn AppContext) {
        let buffer = self.con.buffer_mut();
        buffer.fill(None, Some(RGBA::rgba(255, 0, 255, 255)), None);

        self.skull.blit(buffer, 0, 0, None);

        draw::plain(buffer)
            .align(TextAlign::Center)
            .fg(RGBA::rgb(0, 128, 255))
            .print_lines(30, 10, "This image comes\nfrom a png file.");

        self.con.render(app.gl())
    }
}

fn main() {
    let app = AppBuilder::new(768, 1024)
        .title("SubCell Resolution Example")
        .font(FONT)
        .build();
    app.run::<MyRoguelike>();
}
