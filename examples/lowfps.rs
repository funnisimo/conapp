use conapp::*;

/*
This example shows how you can lower the number of frames per second to limit CPU consumption
using the max_fps field in the AppOptions parameter.
Note that this has no effect on the tickrate at which update() is called which still is 60 times per second.
You can lower the tickrate by calling your world updating code only once every n calls.
*/

const FONT: &str = "resources/terminal_8x8.png";

const RED: RGBA = RGBA::rgba(200, 64, 64, 255);

fn to_color(name: &str) -> Option<RGBA> {
    match name {
        "red" => Some(RED),
        _ => to_rgba(name),
    }
}

struct MyRoguelike {
    con: Console,
}

impl ScreenCreator for MyRoguelike {
    fn create(app: &mut AppContext) -> Box<dyn Screen> {
        let font = app.load_font(FONT);
        let con = Console::new(80, 50, font);

        Box::new(MyRoguelike { con })
    }
}

impl Screen for MyRoguelike {
    fn render(&mut self, app: &mut AppContext) {
        let fps = app.fps();

        let buffer = self.con.buffer_mut();
        let buf_size = buffer.size();

        draw::colored(buffer)
            .align(TextAlign::Center)
            .to_rgba(&to_color)
            .print(
                (buf_size.0 / 2) as i32,
                (buf_size.1 / 2) as i32,
                &format!("Frames since last second : #[red]{}", fps),
            );

        self.con.render(app.gl())
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Low FPS Example")
        .font(FONT)
        .fps(10)
        .build();
    app.run::<MyRoguelike>();
}
