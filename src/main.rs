use data::{Format, Screenshot};
use druid::{commands, AppDelegate, Env, Target, Command, DelegateCtx, Handled, WindowDesc, AppLauncher, theme::{BUTTON_DARK, BUTTON_LIGHT, WINDOW_BACKGROUND_COLOR}, Color};
use im::Vector;
// use saver::read_stored;
use ui::ui_builder;
use image::*;

mod ui;
mod data;
// mod saver;

// commentino molesto
//ciao

// struct Delegate;

fn main() {
    //finestra principale che si apre quando lancio il programma
    let main_window = WindowDesc::new(ui_builder())
        .title("Screen Grabbing")
        .window_size((800., 600.));

    
    let todo_state = Screenshot::new("".to_string(), Format::MainFormat, "".to_string());

    //apre effettivamente finestra con le proprietà definite
    AppLauncher::with_window(main_window)
        .configure_env(|env, _state| {
            env.set(BUTTON_DARK, Color::rgba8(100, 100, 120, 0));
            env.set(BUTTON_LIGHT, Color::rgba8(100, 100, 120, 100));
            env.set(WINDOW_BACKGROUND_COLOR, Color::rgba8(15, 72, 111,1));
        })
        // .delegate(Delegate)
        .launch(todo_state)
        .expect("Failed to start")
}

// impl AppDelegate<ImageBuffer<Rgba<u8>, Vec<u8>>> for Delegate { 
//     fn command( 
//         &mut self, 
//         _ctx: &mut DelegateCtx, 
//         _target: Target, 
//         cmd: &Command, 
//         data: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, 
//         _env: &Env, 
//     ) -> Handled { 
//         if let Some(file_info) = cmd.get(commands::SAVE_FILE_AS) { 
//             if let Some(image_data) data.
//             if let Err(e) = std::fs::write(file_info.path(), &data[..]) { 
//                 println!("Error writing file: {e}"); 
//             } 
//             return Handled::Yes; 
//         } 
//         // if let Some(file_info) = cmd.get(commands::OPEN_FILE) { 
//         //     match std::fs::read_to_string(file_info.path()) { 
//         //         Ok(s) => { 
//         //             let first_line = s.lines().next().unwrap_or(""); 
//         //             *data = first_line.to_owned(); 
//         //         } 
//         //         Err(e) => { 
//         //             println!("Error opening file: {e}"); 
//         //         } 
//         //     } 
//         //     return Handled::Yes; 
//         // } 
//         Handled::No 
//     } 
// }
