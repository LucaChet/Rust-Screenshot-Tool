use chrono;
use druid::widget::{
    Button, Checkbox, ClipBox, Controller, Either, Flex, Label, LineBreaking, List, Padding,
    Painter, RadioGroup, ZStack, TextBox
};
use druid::Command;
use druid::{
    Code, Color, Data, Env, Event, EventCtx, Lens, LocalizedString, Menu, MenuItem, Point,
    UnitPoint, Widget, WidgetExt
};
use screenshots::Screen;
use std::ops::Index;
use std::time::{Duration, Instant, SystemTime};

use druid_widget_nursery::DropdownSelect;

use crate::data::{Format, Screenshot};
// use crate::saver::Saver;

//albero
pub fn ui_builder() -> impl Widget<Screenshot> {
    Flex::column()
    .with_child(
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
                let start = Instant::now();
                let screens = Screen::all().unwrap();

                for screen in screens {
                    println!("capturer {screen:?}");
                    let mut image = screen.capture().unwrap();
                    let mut time = chrono::offset::Utc::now().to_string();
                    // match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH){
                    //     Ok(n) => time = DurationString::from(n).into(),
                    //     Err(_) => panic!("conversione errata!"),
                    // }

                    data.format = Format::Png;    
                    data.name = time + &data.format.to_string();
                    
                    // image
                    //     .save(format!("target/{}.png", screen.display_info.id))
                    //     .unwrap();
                }

                // let screen = Screen::from_point(100, 100).unwrap();
                // println!("capturer {screen:?}");

                // let image = screen.capture_area(300, 300, 300, 300).unwrap();
                // image.save("target/capture_display_with_point.png").unwrap();
                // println!("tempo di esecuzione: {:?}", start.elapsed());
            }),
        ))
    .with_child(

    Flex::row()
    .with_child(Label::new(|data: &Screenshot, _: &Env| data.name.clone()))
    .with_flex_spacer(0.1)
    .with_child(DropdownSelect::new(vec![
                ("Empty", Format::Empty),
                ("Png", Format::Png),
                ("Jpg", Format::Jpg),
                ("Gif", Format::Gif),
            ]).lens(Screenshot::format),)
    )

    
    

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

// struct Enter;

// impl<W: Widget<TodoState>> Controller<TodoState, W> for Enter {
//     fn event(
//         &mut self,
//         child: &mut W,
//         ctx: &mut EventCtx,
//         event: &druid::Event,
//         data: &mut TodoState,
//         env: &Env,
//     ) {
//         if let Event::KeyUp(key) = event {
//             if key.code == Code::Enter {
//                 if data.new_name.trim() != "" {
//                     let name = data.new_name.clone();
//                     data.new_name = "".to_string();
//                     data.todos.push(Screenshot {
                        
//                     });
//                 }
//             }
//         }
//         child.event(ctx, event, data, env)
//     }

//     fn lifecycle(
//         &mut self,
//         child: &mut W,
//         ctx: &mut druid::LifeCycleCtx,
//         event: &druid::LifeCycle,
//         data: &TodoState,
//         env: &Env,
//     ) {
//         child.lifecycle(ctx, event, data, env)
//     }

//     fn update(
//         &mut self,
//         child: &mut W,
//         ctx: &mut druid::UpdateCtx,
//         old_data: &TodoState,
//         data: &TodoState,
//         env: &Env,
//     ) {
//         child.update(ctx, old_data, data, env)
//     }
// }
