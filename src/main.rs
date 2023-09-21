use data::{Format, Screenshot};
use controller::*;

use druid::{WindowDesc, AppLauncher, theme::{BUTTON_DARK, BUTTON_LIGHT, WINDOW_BACKGROUND_COLOR}, Color};
// use saver::read_stored;
use ui::ui_builder;

mod ui;
mod data;
mod controller;
// mod saver;

// commentino molesto
//ciao


fn main() {
    //finestra principale che si apre quando lancio il programma
    let main_window = WindowDesc::new(ui_builder())
        .title("Screen Grabbing")
        .window_size((700., 250.));

    
    let todo_state = Screenshot::new("".to_string(), Format::Png, "".to_string());

    //apre effettivamente finestra con le proprietà definite
    AppLauncher::with_window(main_window)
        .configure_env(|env, _state| {
            env.set(BUTTON_DARK, Color::rgba8(100, 100, 120, 0));
            env.set(BUTTON_LIGHT, Color::rgba8(100, 100, 120, 100));
            env.set(WINDOW_BACKGROUND_COLOR, Color::rgba8(15, 72, 111,1));
            // env.set(theme::, Color::rgba8(15, 72, 111,1));
        })
        .delegate(Delegate)
        .launch(todo_state)
        .expect("Failed to start")
}
