use chrono;
use druid::widget::{
    FillStrat, FlexParams, CrossAxisAlignment, Button, Controller, Either, Flex, Image, Label, Padding, TextBox, ZStack,
};
use druid::{
    piet::{InterpolationMode},
    WidgetPod, commands, Code, Color, Data, Env, Event, EventCtx, FileDialogOptions, FileSpec, Lens,
    LocalizedString, Menu, MenuItem, Point, UnitPoint, Widget, WidgetExt, ImageBuf,
};
use druid::{Command};
use screenshots::Screen;
use std::ops::Index;
use std::time::{Duration, Instant, SystemTime};

use druid_widget_nursery::DropdownSelect;

use crate::data::{Format, Screenshot};
use image::*;
// use crate::saver::Saver;



//albero
pub fn ui_builder() -> impl Widget<Screenshot> {
    let mut is_editing = false;

    let mut col = Flex::column().with_child(
        Flex::row()
            // .with_flex_child(
            //     nameBox::new()
            //         .lens(TodoState::new_name)
            //         .expand_width()
            //         .controller(Enter {}),
            //     1.,
            // )
            .with_child(
                Button::new("SCREEN 📷").on_click(|_ctx, data: &mut Screenshot, _env| {
                    // let start = Instant::now();
                    let screens = Screen::all().unwrap();

                    //CIRO
                    // let screens2 = screenshots::DisplayInfo::all();
                    // let display_info = match screens2{
                    //     Err(why) => return println!("error: {}", why),
                    //     Ok(info) => info,
                    // };

                    // let image2 = screenshots::Screen::new(&display_info[0]);

                    /////////////////////////////////////////////////////////////////////////////////////

                    // for screen in screens {
                    // println!("capturer {screen:?}");
                    let image: ImageBuffer<Rgba<u8>, Vec<u8>> = screens[0].capture().unwrap();
                    let time: String = chrono::offset::Utc::now().to_string();

                    data.format = Format::MainFormat; //default
                    data.name = time;
                    data.name = data
                        .name
                        .replace(".", "-")
                        .replace(":", "-")
                        .replace(" ", "_");
                    data.name += &data.format.to_string();

                    // image
                    //     .save(format!("target/screens/{}.png", data.name))
                    //     .unwrap();

                    // }

                    // let screen = Screen::from_point(100, 100).unwrap();
                    // println!("capturer {screen:?}");

                    // let image = screen.capture_area(300, 300, 300, 300).unwrap();
                    // image.save("target/capture_display_with_point.png").unwrap();
                    // println!("tempo di esecuzione: {:?}", start.elapsed());
                }),
            ),
    );

    let mut row = Flex::row();

    // let label = Label::new(|data: &Screenshot, _: &Env| {
    //     format!("{}{}", data.name, data.format.to_string())
    // });

    let button_modifica =
        Button::new("Modifca").on_click(|ctx: &mut EventCtx, data: &mut Screenshot, _env| {
            data.name = data.new_name.clone();
            data.new_name = "".to_string();
            Screenshot::toggle_textbox_state(data);
            ctx.request_update();
        });
    
    // let text_box = TextBox::new()
    //     .lens(Screenshot::new_name)
    //     .disabled_if(|data: &Screenshot, _: &Env| data.editing_name == false)
    //     .controller(Enter {});

        
    // Creiamo un widget Either che può essere o una Label o una TextBox in base allo stato.
    let either_widget = Either::new(
        |data: &Screenshot, _: &Env| data.editing_name,
        TextBox::new().lens(Screenshot::new_name).controller(Enter {}),
        Label::new(|data: &Screenshot, _: &Env| format!("{}{}", data.name, data.format.to_string())),
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
            let rs = FileSpec::new("gif", &["gif"]);
            let txt = FileSpec::new("png", &["png"]);
            let other = FileSpec::new("Bogus file", &["foo", "bar", "baz"]);
            let save_dialog_options = FileDialogOptions::new()
                .allowed_types(vec![rs, txt, other])
                .default_type(txt)
                .default_name(data.name.clone())
                .name_label("Target")
                .title("Choose a target for this lovely file")
                .button_text("Export");

            ctx.submit_command(druid::commands::SHOW_SAVE_PANEL.with(save_dialog_options.clone()))
        }).align_right();

    row.add_child(either_widget);
    row.add_child(button_modifica);
    // row.add_child(text_box);
    // row.add_flex_spacer(0.1);

    let mut row2 = Flex::row();
    row2.add_child(dropdown);
    row2.add_child(button_save);

    col.add_default_spacer();
    // col.add_child( 
    //     Image::new( 
    //         ImageBuf::from_data(include_bytes!("target/screens/2023-09-09_15-51-15-610481200_UTC.png")).unwrap(), 
    //     )
    //     .fill_mode(FillStrat::Fill) 
    //     .interpolation_mode(InterpolationMode::Bilinear), 
    // );
    // col.add_child(row);
    // col.add_child(row2);
    // col

    ZStack::new(col.with_flex_child(row, FlexParams::new(1.0, CrossAxisAlignment::Start)))
        .with_aligned_child(Padding::new(5., row2), UnitPoint::BOTTOM_RIGHT)

    // .with_child(TextBox::new()
    //     .with_placeholder("file_name")
    //     // .with_placeholder(|data: &Screenshot, _: &Env| format!("{}{}", data.name, data.format.to_string()))
    //     .lens(Screenshot::name))
    //         .with_child(Label::new(|data: &Screenshot, _: &Env| {
    //             format!("{}{}", data.name, data.format.to_string())
    //         }))

    //         .with_child(Button::new("Modifca").on_click(|ctx: &mut EventCtx, data: &mut Screenshot, _env|{
    //             data.editing_name = true;
    //             ctx.request_update();
    //         })
    //         )
    //         .with_child(
    //             TextBox::new()
    //                 .lens(Screenshot::new_name)
    //                 // .expand_width()
    //                 // .padding(8.0),

    //                 // .event(ctx, event, data, env)
    //         )
    //         .with_flex_spacer(0.1)
    //         .with_child(
    //             DropdownSelect::new(vec![
    //                 ("MainFormat", Format::MainFormat),
    //                 ("Png", Format::Png),
    //                 ("Jpg", Format::Jpg),
    //                 ("Gif", Format::Gif),
    //             ])
    //             .lens(Screenshot::format)
    //             .disabled_if(|data: &Screenshot, _: &Env| data.name == ""),
    //         )
    //         .with_child(
    //             Button::new("SAVE")
    //                 .disabled_if(|data: &Screenshot, _: &Env| data.name == "")
    //                 .on_click(move |ctx: &mut EventCtx, data: &mut Screenshot, _env| {
    //                     let rs = FileSpec::new("gif", &["gif"]);
    //                     let txt = FileSpec::new("png", &["png"]);
    //                     let other = FileSpec::new("Bogus file", &["foo", "bar", "baz"]);
    //                     let save_dialog_options = FileDialogOptions::new()
    //                         .allowed_types(vec![rs, txt, other])
    //                         .default_type(txt)
    //                         .default_name(data.name.clone())
    //                         .name_label("Target")
    //                         .title("Choose a target for this lovely file")
    //                         .button_text("Export");

    //                     ctx.submit_command(
    //                         druid::commands::SHOW_SAVE_PANEL.with(save_dialog_options.clone()),
    //                     )
    //                 }),
    //         ),
    // )


    // let todos = List::new(|| {
    //     let bg = Color::rgba8(0, 0, 0, 50);

    // Flex::row()
    //     .with_child(Label::new(|data: &Screenshot, _: &Env| data.name.clone()))
    //     .with_default_spacer()
    //     // .with_child(Checkbox::new("png").lens(TodoItem::checked))
    //     .with_flex_spacer(0.1)
    //     .with_child(DropdownSelect::new(vec![
    //         ("Png", Screenshot::new_with_values("daniel".to_string(), Format::Png)),
    //         ("Jpg",Screenshot::new_with_values("luca".to_string(), Format::Jpg)),
    //         ("Gif", Screenshot::new_with_values("deraj".to_string(), Format::Gif)),
    //     ]))
    // .with_child(Button::new("png").on_click(
    //     |ctx: &mut EventCtx, data: &mut Screenshot, _env| {
    //         let data_clone = data.clone();
    //         // let mouse_position = ctx.window().get_position();
    //         let menu: Menu<TodoState> =
    //             Menu::empty().entry(MenuItem::new("Remove").on_activate(
    //                 move |_, main_data: &mut TodoState, _| {
    //                     let location = main_data.todos.iter().position( |n| n == &data_clone).unwrap();
    //                     main_data.todos.remove(location);
    //                 },
    //             ));
    //         ctx.show_context_menu(menu, Point::new(0., 0.))
    //     },
    // ))
    // .with_child(Button::new("jpg").on_click(
    //     |ctx: &mut EventCtx, data: &mut Screenshot, _env| {
    //         let data_clone = data.clone();
    //         let menu: Menu<TodoState> =
    //             Menu::empty().entry(MenuItem::new("Remove").on_activate(
    //                 move |_, main_data: &mut TodoState, _| {
    //                     let location = main_data.todos.iter().position( |n| n == &data_clone).unwrap();
    //                     main_data.todos.remove(location);
    //                 },
    //             ));
    //         ctx.show_context_menu(menu, Point::new(0., 0.))
    //     },
    // ))
    // .with_child(Button::new("gif").on_click(
    //     |ctx: &mut EventCtx, data: &mut Screenshot, _env| {
    //         let data_clone = data.clone();
    //         let menu: Menu<TodoState> =
    //             Menu::empty().entry(MenuItem::new("Remove").on_activate(
    //                 move |_, main_data: &mut TodoState, _| {
    //                     let location = main_data.todos.iter().position( |n| n == &data_clone).unwrap();
    //                     main_data.todos.remove(location);
    //                 },
    //             ));
    //         ctx.show_context_menu(menu, Point::new(0., 0.))
    //     },
    // ))
    // .with_child(Button::new("raw").on_click(
    //     |ctx: &mut EventCtx, data: &mut Screenshot, _env| {
    //         let data_clone = data.clone();
    //         let menu: Menu<TodoState> =
    //             Menu::empty().entry(MenuItem::new("Remove").on_activate(
    //                 move |_, main_data: &mut TodoState, _| {
    //                     let location = main_data.todos.iter().position( |n| n == &data_clone).unwrap();
    //                     main_data.todos.remove(location);
    //                 },
    //             ));
    //         ctx.show_context_menu(menu, Point::new(0., 0.))
    //     },
    // ))
    //         .background(bg)
    // })
    // .lens(TodoState::todos)
    // .scroll()
    // .vertical();

    // let clear_complete = Button::new("Clear Completed");
    //     // .on_click(|_, data: &mut TodoState, _| data.todos.retain(|item| !item.checked));

    // ZStack::new(Flex::column().with_child(header).with_flex_child(screen, 1.))
    //     .with_aligned_child(Padding::new(5., clear_complete), UnitPoint::BOTTOM_RIGHT)
}

struct Enter;

impl<W: Widget<Screenshot>> Controller<Screenshot, W> for Enter {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &druid::Event,
        data: &mut Screenshot,
        env: &Env,
    ) {
        if let Event::KeyUp(key) = event {
            if key.code == Code::Enter {
                if data.new_name.trim() != "" {
                    data.name = data.new_name.clone();
                    data.new_name = "".to_string();
                    Screenshot::toggle_textbox_state(data);
                }
            }
        }
        child.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &Screenshot,
        env: &Env,
    ) {
        child.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut druid::UpdateCtx,
        old_data: &Screenshot,
        data: &Screenshot,
        env: &Env,
    ) {
        child.update(ctx, old_data, data, env)
    }
}
