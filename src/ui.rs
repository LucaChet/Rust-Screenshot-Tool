use chrono;
use druid::widget::{
    Button, Controller, CrossAxisAlignment, Either, FillStrat, Flex, FlexParams, Image, Label,
    Padding, TextBox, ZStack,
};
use druid::{
    lens, piet::InterpolationMode, Code, Data, Env, Event, EventCtx, FileDialogOptions, FileSpec,
    ImageBuf, Lens, Point, UnitPoint, Widget, WidgetExt, WindowDesc, WindowState, WindowLevel,
};

use screenshots::Screen;
use std::ops::Index;
use std::time::{Duration, Instant, SystemTime};

use druid_widget_nursery::DropdownSelect;

use crate::data::*;
use image::*;
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
                    // current.set_window_state(WindowState::Minimized);
                    // current.hide();
                    // current.set_always_on_top(false);
                    // current.set_size(druid::Size::new(0.0, 0.0));
                    // current.set_position(Point::new(0.0, 0.0));
                    
                    data.window_minimized = true;
                    let new_win = WindowDesc::new(empty_window())
                        .show_titlebar(false)
                        .transparent(true)
                        .window_size((width, height))
                        .resizable(true)
                        .set_position((0.0, 0.0))
                        .set_always_on_top(true);
                        
                    ctx.new_window(new_win);
                    // current.show();
                    // data.do_screen(ctx);
                    
                    // current.set_window_state(WindowState::Restored);
                },
            ))
            .with_child(Button::new("Capture Area 🖱️").on_click(
                move |ctx: &mut EventCtx, data: &mut Screenshot, _env| {
                    let mut current = ctx.window().clone();
                    current.set_window_state(WindowState::Minimized);
                    data.window_minimized = true;
                    let new_win = WindowDesc::new(draw_rect())
                        .show_titlebar(false)
                        .transparent(true)
                        .window_size((width, height))
                        .resizable(false)
                        .set_position((0.0, 0.0));
                    ctx.new_window(new_win);
                    // data.area = SelectedArea::new();
                    // current.set_window_state(WindowState::Restored);
                },
            )),
    );

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
            format!("{}{}", data.name, data.format.to_string())
        }),
    );

    let dropdown = DropdownSelect::new(vec![
        ("MainFormat", Format::MainFormat),
        ("Png", Format::Png),
        ("Jpg", Format::Jpg),
        ("Gif", Format::Gif),
    ])
    .lens(Screenshot::format)
    .disabled_if(|data: &Screenshot, _: &Env| data.name == "")
    .align_right();

    let button_save = Button::new("SAVE")
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

            let default_name = format!("{}{}", data.name.clone(), data.format.to_string());
            let save_dialog_options = FileDialogOptions::new()
                .allowed_types(formats)
                .default_type(FileSpec::new("png", &["png"]))
                .default_name(default_name)
                .name_label("Target")
                .title("Choose a target for this lovely file")
                .button_text("Export");

            ctx.submit_command(druid::commands::SHOW_SAVE_PANEL.with(save_dialog_options.clone()))
        })
        .align_right();

    row.add_child(screen_name);
    row.add_child(button_modifica);
    row.add_child(gestisci_screen);

    let mut row2 = Flex::row();
    row2.add_child(dropdown);
    row2.add_child(button_save);

    col.add_default_spacer();

    // // col.add_child(row);
    // col.add_child(row2);
    // col

    ZStack::new(col.with_flex_child(row, FlexParams::new(1.0, CrossAxisAlignment::Start)))
        .with_aligned_child(Padding::new(5., row2), UnitPoint::BOTTOM_RIGHT)
}
