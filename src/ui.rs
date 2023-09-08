use chrono;
use druid::widget::{
    Button, Checkbox, ClipBox, Controller, Either, Flex, Label, LineBreaking, List, Padding,
    Painter, RadioGroup, TextBox, ZStack,
};
use druid::Command;
use druid::{
    Code, Color, Data, Env, Event, EventCtx, Lens, LocalizedString, Menu, MenuItem, Point,
    UnitPoint, Widget, WidgetExt,
};
use screenshots::Screen;
use std::time::{Duration, Instant, SystemTime};

use druid_widget_nursery::DropdownSelect;

use crate::data::{Format, TodoItem, TodoState};
use crate::saver::Saver;

//albero
pub fn ui_builder() -> impl Widget<TodoState> {
    let header = Flex::row()
        // .with_flex_child(
        //     TextBox::new()
        //         .lens(TodoState::new_text)
        //         .expand_width()
        //         .controller(Enter {}),
        //     1.,
        // )
        .with_child(
            Button::new("SCREEN 📷").on_click(|_ctx, data: &mut TodoState, _env| {
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

                    data.todos.push_back(TodoItem {
                        checked: false,
                        text: time,
                        format: Format::Png,
                    });
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
        )
        .with_child(Saver {});

    let todos = List::new(|| {
        let bg = Color::rgba8(0, 0, 0, 50);

        Flex::row()
            .with_child(Label::new(|data: &TodoItem, _: &Env| data.text.clone()))
            .with_default_spacer()
            // .with_child(Checkbox::new("png").lens(TodoItem::checked))
            .with_child(DropdownSelect::new(vec![
                ("Png", Format::Png),
                ("Jpg", Format::Jpg),
                ("Gif", Format::Gif),
            ])).lens(TodoState::todos)
            .with_flex_spacer(0.1)
            .with_child(Button::new("png").on_click(
                |ctx: &mut EventCtx, data: &mut TodoItem, _env| {
                    let data_clone = data.clone();
                    // let mouse_position = ctx.window().get_position();
                    let menu: Menu<TodoState> =
                        Menu::empty().entry(MenuItem::new("Remove").on_activate(
                            move |_, main_data: &mut TodoState, _| {
                                let location = main_data.todos.index_of(&data_clone).unwrap();
                                main_data.todos.remove(location);
                            },
                        ));
                    ctx.show_context_menu(menu, Point::new(0., 0.))
                },
            ))
            .with_child(Button::new("jpg").on_click(
                |ctx: &mut EventCtx, data: &mut TodoItem, _env| {
                    let data_clone = data.clone();
                    let menu: Menu<TodoState> =
                        Menu::empty().entry(MenuItem::new("Remove").on_activate(
                            move |_, main_data: &mut TodoState, _| {
                                let location = main_data.todos.index_of(&data_clone).unwrap();
                                main_data.todos.remove(location);
                            },
                        ));
                    ctx.show_context_menu(menu, Point::new(0., 0.))
                },
            ))
            .with_child(Button::new("gif").on_click(
                |ctx: &mut EventCtx, data: &mut TodoItem, _env| {
                    let data_clone = data.clone();
                    let menu: Menu<TodoState> =
                        Menu::empty().entry(MenuItem::new("Remove").on_activate(
                            move |_, main_data: &mut TodoState, _| {
                                let location = main_data.todos.index_of(&data_clone).unwrap();
                                main_data.todos.remove(location);
                            },
                        ));
                    ctx.show_context_menu(menu, Point::new(0., 0.))
                },
            ))
            .with_child(Button::new("raw").on_click(
                |ctx: &mut EventCtx, data: &mut TodoItem, _env| {
                    let data_clone = data.clone();
                    let menu: Menu<TodoState> =
                        Menu::empty().entry(MenuItem::new("Remove").on_activate(
                            move |_, main_data: &mut TodoState, _| {
                                let location = main_data.todos.index_of(&data_clone).unwrap();
                                main_data.todos.remove(location);
                            },
                        ));
                    ctx.show_context_menu(menu, Point::new(0., 0.))
                },
            ))
            .background(bg)
    })
    .lens(TodoState::todos)
    .scroll()
    .vertical();

    let clear_complete = Button::new("Clear Completed")
        .on_click(|_, data: &mut TodoState, _| data.todos.retain(|item| !item.checked));

    ZStack::new(Flex::column().with_child(header).with_flex_child(todos, 1.))
        .with_aligned_child(Padding::new(5., clear_complete), UnitPoint::BOTTOM_RIGHT)
}

struct Enter;

impl<W: Widget<TodoState>> Controller<TodoState, W> for Enter {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &druid::Event,
        data: &mut TodoState,
        env: &Env,
    ) {
        if let Event::KeyUp(key) = event {
            if key.code == Code::Enter {
                if data.new_text.trim() != "" {
                    let text = data.new_text.clone();
                    data.new_text = "".to_string();
                    data.todos.push_front(TodoItem {
                        checked: false,
                        text,
                    });
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
        data: &TodoState,
        env: &Env,
    ) {
        child.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut druid::UpdateCtx,
        old_data: &TodoState,
        data: &TodoState,
        env: &Env,
    ) {
        child.update(ctx, old_data, data, env)
    }
}
