use conapp::*;

const FONT: &str = "resources/terminal_8x8.png";

const _WHITE: RGBA = RGBA::rgba(255, 255, 255, 255);

struct MyRoguelike {
    con: Console,
    skull: Image,
    angle: f32,
    scale_time: f32,
}

impl ScreenCreator for MyRoguelike {
    fn create(app: &mut dyn AppContext) -> Box<dyn Screen> {
        let font = app.get_font(FONT).expect(&format!(
            "Trying to use font that was not loaded.  Add this font to the AppBuilder - {}",
            FONT
        ));

        let con = Console::new(80, 50, font);

        Box::new(MyRoguelike {
            con,
            skull: Image::new("resources/skull.png"),
            angle: 0.0,
            scale_time: 0.0,
        })
    }
}

impl Screen for MyRoguelike {
    fn update(&mut self, _api: &mut dyn AppContext, _ms: f32) -> ScreenResult {
        self.angle += 0.01;
        self.scale_time += 0.01;
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut dyn AppContext) {
        let buffer = self.con.buffer_mut();
        let scale = self.scale_time.cos();
        buffer.fill(None, None, Some((0, 0, 0, 255).into()));

        self.skull.blit_ex(
            buffer,
            (buffer.get_width() / 2) as f32,
            (buffer.get_height() / 2) as f32,
            scale,
            scale,
            self.angle,
            None,
        );

        self.con.render(app.gl())
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Image Example")
        .font(FONT)
        .build();
    app.run::<MyRoguelike>();
}
