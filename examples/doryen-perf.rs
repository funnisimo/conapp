use conapp::*;

const FONT: &str = "resources/terminal_8x8.png";

const WHITE: RGBA = RGBA::rgb(255, 255, 255);
const BLACK: RGBA = RGBA::rgb(0, 0, 0);

struct RNG {
    seed: u64,
}

impl RNG {
    pub fn new() -> Self {
        RNG { seed: 0xdead_beef }
    }

    fn next_u64(&mut self) -> u64 {
        self.seed = 214_013u64.wrapping_mul(self.seed).wrapping_add(2_531_011);
        self.seed
    }
}

struct PerfTest {
    con: Console,
    rng: RNG,
}

impl ScreenCreator for PerfTest {
    fn create(app: &mut dyn AppContext) -> Box<dyn Screen> {
        let font = app.get_font(FONT).expect(&format!(
            "Trying to use font that was not loaded.  Add this font to the AppBuilder - {}",
            FONT
        ));

        let con = Console::new(80, 50, font);
        let mut screen = Box::new(PerfTest {
            con,
            rng: RNG::new(),
        });

        screen.resize(app);
        screen
    }
}

impl Screen for PerfTest {
    fn render(&mut self, app: &mut dyn AppContext) {
        // let con = &mut self.con;

        let con_width = self.con.get_width();
        let con_height = self.con.get_height();

        let buffer = self.con.buffer_mut();

        for y in 0..con_height as i32 {
            for x in 0..con_width as i32 {
                let val = self.rng.next_u64();
                buffer.back(
                    x,
                    y,
                    RGBA::rgba(
                        (val & 0xFF) as u8,
                        ((val >> 8) & 0x5F) as u8,
                        ((val >> 16) & 0x3F) as u8,
                        255,
                    ),
                );
                buffer.fore(
                    x,
                    y,
                    RGBA::rgba(
                        ((val >> 16) & 0xFF) as u8,
                        ((val >> 24) & 0xFF) as u8,
                        ((val >> 32) & 0xFF) as u8,
                        255,
                    ),
                );
                buffer.glyph(x, y, ((val >> 40) & 0xFF) as u32);
            }
        }
        draw::frame(buffer)
            .fg(WHITE)
            .bg(BLACK)
            .fill(Some(' ' as u32), None, Some(BLACK))
            .draw(
                (con_width / 2 - 10) as i32,
                (con_height / 2 - 2) as i32,
                20,
                5,
            );

        let fps = app.fps();

        draw::colored(buffer)
            .align(TextAlign::Center)
            .fg(WHITE)
            .print(
                (con_width / 2) as i32,
                (con_height / 2) as i32,
                &format!("{} fps", fps),
            );

        self.con.render(app.gl());
    }

    fn resize(&mut self, api: &mut dyn AppContext) {
        let new_width = api.get_screen_size().0 / 8;
        let new_height = api.get_screen_size().1 / 8;
        self.con.resize(new_width, new_height);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("doryen-rs performance test")
        .font(FONT)
        .vsync(false)
        .build();

    app.run::<PerfTest>();
}