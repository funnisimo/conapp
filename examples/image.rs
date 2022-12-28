use conapp::*;
use std::cell::RefCell;
use std::rc::Rc;

const FONT: &str = "resources/terminal_8x8.png";

const _WHITE: RGBA = RGBA::rgba(255, 255, 255, 255);

struct MyRoguelike {
    con: Console,
    skull: Rc<RefCell<Image>>,
    angle: f32,
    scale_time: f32,
}

impl ScreenCreator for MyRoguelike {
    fn create(app: &mut dyn AppContext) -> Box<dyn Screen> {
        let font = app.get_font(FONT);
        let con = Console::new(80, 50, font);

        Box::new(MyRoguelike {
            con,
            skull: app.get_image("resources/skull.png"),
            angle: 0.0,
            scale_time: 0.0,
        })
    }
}

impl Screen for MyRoguelike {
    fn update(&mut self, _api: &mut dyn AppContext, _ms: f64) -> ScreenResult {
        self.angle += 0.01;
        self.scale_time += 0.01;
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut dyn AppContext) {
        let buffer = self.con.buffer_mut();
        let buf_size = buffer.size();
        let scale = self.scale_time.cos();
        buffer.fill(None, None, Some((0, 0, 0, 255).into()));

        draw::image(buffer).blit_ex(
            (buf_size.0 / 2) as f32,
            (buf_size.1 / 2) as f32,
            scale,
            scale,
            self.angle,
            &self.skull,
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
