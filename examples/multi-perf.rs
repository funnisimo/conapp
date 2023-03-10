use conapp::*;

const FONTA: &str = "resources/terminal_8x8.png";
const FONTB: &str = "resources/Runeset_24x24.png";

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
    left: Console,
    right: Console,
    rng: RNG,
}

impl PerfTest {
    fn new() -> Box<Self> {
        let left = Console::new(20, 25, FONTA).with_extents(0.0, 0.0, 0.5, 1.0);
        let right = Console::new(40, 50, FONTB).with_extents(0.5, 0.0, 1.0, 1.0);

        Box::new(PerfTest {
            left,
            right,
            rng: RNG::new(),
        })
    }
}

impl PerfTest {
    fn render_con(&mut self, is_left: bool, app: &mut AppContext) {
        let con = match is_left {
            true => &mut self.left,
            false => &mut self.right,
        };

        let buffer = con.buffer_mut();
        let con_width = buffer.width();
        let con_height = buffer.height();

        for y in 0..con_height as i32 {
            for x in 0..con_width as i32 {
                let val = self.rng.next_u64();
                buffer.back(
                    x,
                    y,
                    RGBA::rgba(
                        (val & 0xFF) as u8,
                        ((val >> 8) & 0x5F) as u8,
                        ((val >> 16) & 0x5F) as u8,
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

        con.render(app);
    }
}

impl Screen for PerfTest {
    fn render(&mut self, app: &mut AppContext) {
        self.render_con(true, app);
        self.render_con(false, app);
    }

    fn resize(&mut self, api: &mut AppContext) {
        let new_width = api.screen_size().0 / 32;
        let new_height = api.screen_size().1 / 16;

        console(format!(
            "resize - {:?} => {},{}",
            api.screen_size(),
            new_width,
            new_height
        ));

        self.left.resize(new_width, new_height);
        self.right.resize(new_width, new_height);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("doryen-rs performance test")
        .font(FONTA)
        .font(FONTB)
        .vsync(false)
        .fps(0)
        .build();

    app.run_screen(PerfTest::new());
}
