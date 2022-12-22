use conapp::*;

fn main() {
    // create the game window (native) or canvas (web)
    let app = App::new(AppConfig {
        size: (800, 600),
        title: "my game".to_owned(),
        ..AppConfig::default()
    });

    // start game loop
    app.run(move |app: &mut App| {
        for evt in app.events.borrow().iter() {
            // print on stdout (native) or js console (web)
            console(format!("{:?}", evt));
            // exit on key or mouse press
            match evt {
                &AppEvent::KeyUp(_) => {
                    App::exit();
                }
                &AppEvent::MouseUp(_) => {
                    App::exit();
                }
                _ => (),
            }
        }
    });
}
