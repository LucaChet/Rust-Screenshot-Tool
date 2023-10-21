
use druid::text::ParseFormatter;
use druid::widget::{
    Button, CrossAxisAlignment, Either, Flex, FlexParams, Label, Padding,
    Stepper, TextBox, ZStack,
};

use druid::{
    WindowId, Color, Env, EventCtx, FileDialogOptions, FileSpec,
    LocalizedString, Menu, MenuItem, UnitPoint, Widget, WidgetExt,
};


use druid_shell::{HotKey, SysMods, Application};
use druid_widget_nursery::{DropdownSelect, WidgetExt as _};
use image::ImageBuffer;

use crate::controller::*;
use crate::data::*;

// use crate::saver::Saver;

//albero
pub fn ui_builder() -> impl Widget<Screenshot> {
    let mut col = Flex::column().with_child(
        Flex::row()
            .with_child(Button::new(|data: &Screenshot, _:&Env| format!("📷  (Ctrl+{})", data.shortcut.get(&Shortcut::Screenshot).unwrap().to_uppercase().as_str()))
            .stack_tooltip("Screen")
            .on_click(
                |ctx, data: &mut Screenshot, _env| {
                    data.action_screen(ctx);
                },
            ))
            .with_child(Button::new(|data: &Screenshot, _:&Env| format!("🖱️  (Ctrl+{})", data.shortcut.get(&Shortcut::Capture).unwrap().to_uppercase().as_str()))
            .stack_tooltip("Capture Area")
            .on_click(
                |ctx: &mut EventCtx, data: &mut Screenshot, _env| {
                    data.action_capture(ctx);
                },
            )),
    );

    let timer_box = Stepper::new()
        .with_range(0.0, 100.0)
        .with_step(1.0)
        .lens(Screenshot::time_interval);

    let label_timer = Label::new(|data: &Screenshot, _: &Env| {
        format!("⌛Delay timer: {} secondi", data.time_interval)
    });

    let row_timer = Flex::row()
        .with_child(label_timer)
        .with_spacer(1.0)
        .with_child(timer_box);

    let mut row = Flex::row();

    let button_modifica = Either::new(
        |data: &Screenshot, _: &Env| data.screen_fatto,
        Button::new("Modifica nome").on_click(|ctx: &mut EventCtx, data: &mut Screenshot, _env| {
            data.name = data.new_name.clone();
            data.new_name = "".to_string();
            Screenshot::toggle_textbox_state(data);
            ctx.request_update();
        }),
        Label::new(""),
    );

    let gestisci_screen = Either::new(
        |data: &Screenshot, _: &Env| data.screen_fatto,
        Button::new("Gestisci screen").on_click(
            |ctx: &mut EventCtx, data: &mut Screenshot, _env| {
                data.screen_window(ctx);
                ctx.request_update();
            },
        ),
        Label::new(""),
    );

    // Creiamo un widget Either che può essere o una Label o una TextBox in base allo stato.
    let screen_name = Either::new(
        |data: &Screenshot, _: &Env| data.editing_name,
        TextBox::new()
            .lens(Screenshot::new_name)
            .controller(Enter {}),
        Label::new(|data: &Screenshot, _: &Env| {
            if data.name == "" {
                format!("{}", data.name)
            } else {
                format!("{}{}", data.name, data.format.to_string())
            }
        }),
    );

    row.add_child(screen_name);
    row.add_child(button_modifica);
    row.add_child(gestisci_screen);

    // let mut row2 = Flex::row();
    // row2.add_child(dropdown);
    col.add_default_spacer();

    ZStack::new(col.with_flex_child(row, FlexParams::new(1.0, CrossAxisAlignment::Start)))
        .with_aligned_child(Padding::new(5., row_timer), UnitPoint::BOTTOM_RIGHT)
        .controller(HotkeyScreen {flag_ctrl: false})

}

#[allow(unused_assignments)]
pub fn menu(_: Option<WindowId>, _state: &Screenshot, _: &Env) -> Menu<Screenshot>{
    let menu = Menu::empty();

    let mut file = Menu::new(LocalizedString::new("File"));
    file = file
    .entry(
        MenuItem::new(LocalizedString::new("Choose default path..")).on_activate(
            |ctx, _data: &mut Screenshot, _env|{
                let open_dialog_options = FileDialogOptions::new()
                .select_directories()
                .button_text("Open");

                ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()))
            }
        ).dynamic_hotkey(|data, _env| Some(HotKey::new(SysMods::Cmd, data.shortcut.get(&Shortcut::Open).unwrap().as_str())))
    )
    .entry(
        MenuItem::new(LocalizedString::new("Save..")).on_activate(
            |_ctx, data: &mut Screenshot, _env|{
                let image: ImageBuffer<image::Rgba<u8>, Vec<u8>> = ImageBuffer::from_vec(
                    data.img.width() as u32,
                    data.img.height() as u32,
                    data.img.raw_pixels().to_vec(),
                )
                .unwrap();
    
                image
                    .save_with_format(
                        format!(
                            "{}/{}{}",
                            data.default_save_path.clone(),
                            data.name,
                            data.format.to_string()
                        ),
                        image::ImageFormat::Png,
                    )
                    .expect("Errore nel salvataggio automatico!");
            }
        ).enabled_if(|data: &Screenshot, _: &Env| data.name != "")
        .dynamic_hotkey(|data, _env| Some(HotKey::new(SysMods::Cmd, data.shortcut.get(&Shortcut::Save).unwrap().as_str())))
    )
    .entry(
        MenuItem::new(LocalizedString::new("Save as..")).on_activate(
            |ctx, data: &mut Screenshot, _env|{
                let formats = vec![
                    FileSpec::new("jpg", &["jpg"]),
                    FileSpec::new("png", &["png"]),
                    FileSpec::new("gif", &["gif"]),
                    FileSpec::new("pnm", &["pnm"]),
                    FileSpec::new("tga", &["tga"]),
                    FileSpec::new("qoi", &["qoi"]),
                    FileSpec::new("tiff", &["tiff"]),
                    FileSpec::new("webp", &["webp"]),
                    FileSpec::new("bmp", &["bmp"]),
                ];
    
                let default_name = format!("{}{}", data.name, data.format.to_string());
                let save_dialog_options = FileDialogOptions::new()
                    .allowed_types(formats)
                    .default_type(FileSpec::new("png", &["png"]))
                    .default_name(default_name);
                
                ctx.submit_command(druid::commands::SHOW_SAVE_PANEL.with(save_dialog_options.clone()))
            }
        ).enabled_if(|data: &Screenshot, _: &Env| data.name != "")
        .dynamic_hotkey(|data, _env| Some(HotKey::new(SysMods::Cmd, data.shortcut.get(&Shortcut::SaveAs).unwrap().as_str())))
    )
    .entry(MenuItem::new(LocalizedString::new("Customize shortcut..")).on_activate(
        |ctx, data: &mut Screenshot, _env|{
            data.editing_shortcut = true;
            data.new_name = "".to_string();
            ctx.submit_command(SHORTCUT)   
        }
    ).dynamic_hotkey(|data, _env| Some(HotKey::new(SysMods::Cmd, data.shortcut.get(&Shortcut::Customize).unwrap().as_str())))
).separator()
    .entry(
        MenuItem::new(LocalizedString::new("Quit")).on_activate(
            |_ctx, _data: &mut Screenshot, _env|{
                Application::global().quit();
            })
            .dynamic_hotkey(|data, _env| Some(HotKey::new(SysMods::Cmd, data.shortcut.get(&Shortcut::Quit).unwrap().as_str())))
    );

    let mut format = Menu::new(LocalizedString::new("Format"));
    format = format.entry(
        MenuItem::new(LocalizedString::new("Png")).on_activate(
            |_ctx, data: &mut Screenshot, _env|{
                data.format = Format::Png;
            }
        )
    ).entry(
        MenuItem::new(LocalizedString::new("Jpg")).on_activate(
            |_ctx, data: &mut Screenshot, _env|{
                data.format = Format::Jpg;
            }
        )
    ).entry(
        MenuItem::new(LocalizedString::new("Gif")).on_activate(
            |_ctx, data: &mut Screenshot, _env|{
                data.format = Format::Gif;
            }
        )
    ).entry(
        MenuItem::new(LocalizedString::new("Pnm")).on_activate(
            |_ctx, data: &mut Screenshot, _env|{
                data.format = Format::Pnm;
            }
        )
    ).entry(
        MenuItem::new(LocalizedString::new("Tga")).on_activate(
            |_ctx, data: &mut Screenshot, _env|{
                data.format = Format::Tga;
            }
        )
    ).entry(
        MenuItem::new(LocalizedString::new("Qoi")).on_activate(
            |_ctx, data: &mut Screenshot, _env|{
                data.format = Format::Qoi;
            }
        )
    ).entry(
        MenuItem::new(LocalizedString::new("Tiff")).on_activate(
            |_ctx, data: &mut Screenshot, _env|{
                data.format = Format::Tiff;
            }
        )
    ).entry(
        MenuItem::new(LocalizedString::new("Webp")).on_activate(
            |_ctx, data: &mut Screenshot, _env|{
                data.format = Format::Webp;
            }
        )
    ).entry(
        MenuItem::new(LocalizedString::new("Bmp")).on_activate(
            |_ctx, data: &mut Screenshot, _env|{
                data.format = Format::Bmp;
            }
        )
    );

    let mut monitor: Menu<Screenshot> = Menu::new(LocalizedString::new("Monitor"));
    let n_displays = screenshots::DisplayInfo::all().expect("error").len();

    for i in 0..n_displays{
        monitor = monitor.entry(MenuItem::new(format!("display {}",i+1).as_str()).on_activate(
            move |_ctx, data: &mut Screenshot, _env|{
                data.monitor_id = i;
            }
        ).selected_if(move |data, _:&Env|data.monitor_id == i));
    }
    
    menu.entry(file).entry(format).entry(monitor)
}

pub fn modify_shortcut() -> impl Widget<Screenshot> {
    let dropdown = DropdownSelect::new(vec![
        ("Save", Shortcut::Save),
        ("Save as", Shortcut::SaveAs),
        ("Choose default path", Shortcut::Open),
        ("Customize", Shortcut::Customize),
        ("Screenshot", Shortcut::Screenshot),
        ("Capture", Shortcut::Capture),
        ("Quit", Shortcut::Quit),
    ])
    .lens(Screenshot::selected_shortcut)
    .align_right();

    let mut col = Flex::column();
   
    let textbox = 
    Either::new(|data, _: &Env| data.editing_shortcut,
        Label::new(|data: &Screenshot, _: &Env| format!("{}", data.new_name)),
        Label::new(""));

    let row0 = Flex::row().with_child(Label::new("Type the key combination to customize the shortcut
associated with the action\n")).center();

    let row_alert = Flex::row().with_child(Either::new(
        |data: &Screenshot, _: &Env| data.duplicate_shortcut,
        Label::new("⚠️ALREADY EXISTS!⚠️").border(Color::RED, 2.).background(Color::GRAY).center(),
        Label::new(""),
    ));

    let mut row1 = Flex::row();
    row1.add_child(dropdown);
    row1.add_child(textbox);
    col.add_child(row0);
    col.add_child(row1);
    col.add_default_spacer();
    col.add_child(row_alert);
    col.controller(HotKeyController{prec: String::from("")})


}
