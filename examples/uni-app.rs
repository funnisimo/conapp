use conapp::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    main();
    Ok(())
}

fn main() {
    // create the game window (native) or canvas (web)
    let app = App::new(AppConfig {
        size: (800, 600),
        title: "my game".to_owned(),
        ..Default::default()
    });

    // start game loop
    app.run(move |app: &mut App| {
        for evt in app.events.borrow().iter() {
            // print on stdout (native) or js console (web)
            App::print(format!("{:?}", evt));
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
