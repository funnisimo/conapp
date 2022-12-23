use conapp::*;
use std::cell::RefCell;
use std::rc::Rc;

const FONT: &str = "resources/terminal_8x8.png";
const BIG_FONT: &str = "resources/ProjectUtumno_full_32x32.png";

const BLACK: RGBA = RGBA::rgb(0, 0, 0);
const _GRAY: RGBA = RGBA::rgb(128, 128, 128);
const _RED: RGBA = RGBA::rgb(192, 32, 32);
const YELLOW: RGBA = RGBA::rgb(192, 192, 32);

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

struct LoadingScreen {
    con: Console,
    big_font: Option<Rc<RefCell<Font>>>,
}
impl ScreenCreator for LoadingScreen {
    fn create(app: &mut dyn AppContext) -> Box<dyn Screen> {
        let font = app.load_font(FONT);
        let con = Console::new(80, 50, font);

        Box::new(LoadingScreen {
            con,
            big_font: None,
        })
    }
}

impl Screen for LoadingScreen {
    fn setup(&mut self, app: &mut dyn AppContext) {
        self.big_font = Some(app.load_font(BIG_FONT));
    }

    fn update(&mut self, app: &mut dyn AppContext, _frame_time_ms: f32) -> ScreenResult {
        if let Some(ref font) = self.big_font {
            if font.borrow().ready() {
                return ScreenResult::Replace(MainScreen::new(app, font.clone()));
            }
        }
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut dyn AppContext) {
        let buf = self.con.buffer_mut();
        buf.clear(true, true, true);

        buf.fill(Some('.' as u32), Some(YELLOW), Some(BLACK));

        draw::plain(buf).print(1, 1, "Hello Rust World");
        draw::plain(buf).print(1, 2, "Loading a bigger font...");

        self.con.render(app.gl());
    }
}

struct MainScreen {
    con: Console,
    len: u32,
    rng: RNG,
}

impl MainScreen {
    pub fn new(_app: &mut dyn AppContext, font: Rc<RefCell<Font>>) -> Box<MainScreen> {
        let len = font.borrow().len();
        let con = Console::new(80, 40, font);

        Box::new(MainScreen {
            con,
            len,
            rng: RNG::new(),
        })
    }
}

impl Screen for MainScreen {
    fn input(&mut self, _ctx: &mut dyn AppContext, ev: &AppEvent) -> ScreenResult {
        match ev {
            AppEvent::MouseDown(_) => ScreenResult::Pop,
            _ => ScreenResult::Continue,
        }
    }

    fn render(&mut self, app: &mut dyn AppContext) {
        // let screen_pct = app.input().mouse_pct();
        // let cell_pct = self.con.cell_pos(screen_pct);

        let buf = self.con.buffer_mut();

        // buf.clear(true, true, true);

        for y in 0..buf.get_height() as i32 {
            for x in 0..buf.get_width() as i32 {
                if self.rng.next_u64() % 10_u64 == 0 {
                    let glyph = self.rng.next_u64() as u32 % self.len;
                    buf.draw_opt(x, y, Some(glyph), Some(RGBA::rgb(255, 255, 255)), None)
                }
            }
        }

        self.con.render(app.gl());
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Loading Screen Example")
        .font(FONT)
        .build();
    app.run::<LoadingScreen>();
}
