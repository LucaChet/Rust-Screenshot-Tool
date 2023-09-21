use std::fs::File;

use druid::widget::{
    Button, Container, CrossAxisAlignment, Either, Flex, FlexParams, Image, Label, Padding,
    Painter, Stepper, TextBox, ZStack,
};

use druid::{
    commands, AppDelegate, Color, Command, Data, Env, EventCtx, FileDialogOptions, FileSpec,
    ImageBuf, LocalizedString, Menu, MenuItem, Point, RenderContext, UnitPoint, Widget, WidgetExt,
    WindowDesc, WindowState,
};

use druid_widget_nursery::DropdownSelect;
use image::ImageBuffer;

use crate::controller::*;
use crate::data::*;

// use crate::saver::Saver;

//albero
pub fn ui_builder() -> impl Widget<Screenshot> {
    let displays = screenshots::DisplayInfo::all().expect("error");
    let scale = displays[0].scale_factor as f64;
    let width = displays[0].width as f64 * scale;
    let height = displays[0].height as f64 * scale;

    let mut col = Flex::column().with_child(
        Flex::row()
            .with_child(Button::new("SCREEN 📷").on_click(
                move |ctx, data: &mut Screenshot, _env| {
                    let mut current = ctx.window().clone();
                    current.set_window_state(WindowState::Minimized);
                    data.full_screen = true;

                    data.area.start = Point::new(0.0, 0.0);
                    data.area.end = Point::new(0.0, 0.0);
                    data.area.width = 0.0;
                    data.area.heigth = 0.0;
                    data.area.rgba.reset();

                    let new_win = WindowDesc::new(draw_rect())
                        .show_titlebar(false)
                        .transparent(true)
                        .window_size((width, height))
                        .resizable(true)
                        .set_position((0.0, 0.0))
                        .set_always_on_top(true);

                    ctx.new_window(new_win);
                },
            ))
            .with_child(Button::new("Capture Area 🖱️").on_click(
                move |ctx: &mut EventCtx, data: &mut Screenshot, _env| {
                    let mut current = ctx.window().clone();
                    current.set_window_state(WindowState::Minimized);
                    data.full_screen = false;

                    data.area.start = Point::new(0.0, 0.0);
                    data.area.end = Point::new(0.0, 0.0);
                    data.area.width = 0.0;
                    data.area.heigth = 0.0;
                    data.area.rgba.reset();

                    let container = Either::new(
                        |data: &Screenshot, _: &Env| data.flag_transparency,
                        Container::new(draw_rect()).background(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                        Container::new(draw_rect()).background(Color::rgba(0.0, 0.0, 0.0, 0.6)),
                    );

                    let new_win = WindowDesc::new(container)
                        .show_titlebar(false)
                        .transparent(true)
                        .window_size((width, height))
                        .resizable(false)
                        .set_position((0.0, 0.0));

                    ctx.new_window(new_win);
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
    let mut save_row = Flex::row();

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

    let dropdown = DropdownSelect::new(vec![
        ("Png", Format::Png),
        ("Jpg", Format::Jpg),
        ("Gif", Format::Gif),
    ])
    .lens(Screenshot::format)
    .disabled_if(|data: &Screenshot, _: &Env| data.name == "")
    .align_right();

    let button_save_as = Button::new("SAVE AS")
        .disabled_if(|data: &Screenshot, _: &Env| data.name == "")
        .on_click(move |ctx: &mut EventCtx, data: &mut Screenshot, _env| {
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
                // .title("Choose a target for this lovely file")
                // .name_label("Target")
                .default_name(default_name);
            // .button_text("Export");

            ctx.submit_command(druid::commands::SHOW_SAVE_PANEL.with(save_dialog_options.clone()))
        })
        .align_right();

    let button_path = Button::new("Choose Default Path").on_click(
        move |ctx: &mut EventCtx, _data: &mut Screenshot, _env| {
            let open_dialog_options = FileDialogOptions::new()
                .select_directories()
                .button_text("Open");

            ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()))
        },
    );

    let button_save =
        Button::new("Save").on_click(move |_ctx: &mut EventCtx, data: &mut Screenshot, _env| {
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
        });

    row.add_child(screen_name);
    row.add_child(button_modifica);
    row.add_child(gestisci_screen);

    let mut row2 = Flex::row();
    row2.add_child(dropdown);
    save_row.add_child(button_path);
    save_row.add_child(button_save);
    save_row.add_child(button_save_as);

    col.add_default_spacer();

    // col.add_child(label_widget(row_timer, "Delay Timer"));
    // col.add_child(row);
    // col.add_child(row2);
    // col

    ZStack::new(col.with_flex_child(row, FlexParams::new(1.0, CrossAxisAlignment::Start)))
        .with_aligned_child(Padding::new(5., row_timer), UnitPoint::TOP_LEFT)
        .with_aligned_child(Padding::new(5., save_row), UnitPoint::BOTTOM_RIGHT)
        .with_aligned_child(Padding::new(5., row2), UnitPoint::BOTTOM_LEFT)
}
