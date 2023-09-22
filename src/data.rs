use druid::{
    widget::{
        Button, Container, Controller, Either, FillStrat, Flex, Image, Painter, SizedBox, ZStack,
    },
    BoxConstraints, Color, CursorDesc, Data, Env, Event, EventCtx, ImageBuf, LayoutCtx, Lens,
    LifeCycle, LifeCycleCtx, PaintCtx, Point, Rect, RenderContext, Size, TimerToken, UpdateCtx,
    Widget, WidgetExt, WidgetPod, WindowDesc, WindowState,
};
// use druid_shell::{TimerToken};

use crate::controller::*;
use arboard::Clipboard;
use arboard::ImageData;
use image::{codecs::png::PngDecoder, *};
use raster::{transform, Color as rasterColor, Image as rasterImage};
use screenshots::Screen;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use self::selected_area_derived_lenses::heigth;

#[derive(Clone, Data, PartialEq, Debug, Serialize, Deserialize)]
pub enum Format {
    Png,
    Jpg,
    Gif,
}

impl Format {
    pub fn to_string(&self) -> String {
        match self {
            Format::Jpg => ".jpg".to_string(),
            Format::Png => ".png".to_string(),
            Format::Gif => ".gif".to_string(),
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct RgbaArea {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}
impl RgbaArea {
    pub fn reset(&mut self) {
        self.r = 10.0;
        self.g = 10.0;
        self.b = 10.0;
        self.a = 0.4;
    }
}

#[derive(Clone, Data, Lens)]
pub struct SelectedArea {
    pub start: Point,
    pub end: Point,
    pub width: f64,
    pub heigth: f64,
    pub scale: f64,
    pub rgba: RgbaArea,
}
impl SelectedArea {
    pub fn new() -> Self {
        let displays = screenshots::DisplayInfo::all().expect("error");
        let scale = displays[0].scale_factor as f64;
        Self {
            start: Point { x: 0.0, y: 0.0 },
            end: Point { x: 0.0, y: 0.0 },
            width: 0.0,
            heigth: 0.0,
            scale,
            rgba: RgbaArea {
                r: 10.0,
                g: 10.0,
                b: 10.0,
                a: 0.4,
            },
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct ResizedArea{
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
impl ResizedArea{
    pub fn new()->Self{
        Self { x: 0.0, y: 0.0, width: 0.0, height: 0.0 }
    }
}

#[derive(Clone, Data, Lens)]
pub struct Screenshot {
    pub name: String,
    pub format: Format,
    pub new_name: String,
    pub editing_name: bool,
    pub screen_fatto: bool,
    pub img: ImageBuf,
    pub area: SelectedArea,
    pub flag_transparency: bool,
    pub flag_selection: bool, //serve per fare far partire il controller solo dopo aver acquisito l'area
    pub full_screen: bool,
    pub time_interval: f64,
    pub default_save_path: String,
    pub flag_resize: bool,
    pub resized_area: ResizedArea,
}

impl Screenshot {
    pub fn new(name: String, format: Format, newname: String) -> Self {
        Self {
            name,
            format,
            new_name: newname,
            editing_name: false,
            screen_fatto: false,
            img: ImageBuf::empty(),
            area: SelectedArea::new(),
            flag_transparency: false,
            flag_selection: false,
            full_screen: false,
            time_interval: 0.0,
            default_save_path: "C:/Users/Utente/Pictures".to_string(),
            flag_resize: false,
            resized_area: ResizedArea::new(),
        }
    }

    pub fn toggle_textbox_state(data: &mut Screenshot) {
        if data.editing_name {
            data.editing_name = false;
        } else {
            data.editing_name = true;
        }
    }

    pub fn do_screen(&mut self) {
        let screens = Screen::all().unwrap();
        let image: ImageBuffer<Rgba<u8>, Vec<u8>> = screens[0].capture().unwrap();
        let time: String = chrono::offset::Utc::now().to_string();

        self.name = time;
        self.name = self
            .name
            .replace(".", "-")
            .replace(":", "-")
            .replace(" ", "_");
        // self.name += &self.format.to_string();

        self.img = ImageBuf::from_raw(
            image.clone().into_raw(),
            druid::piet::ImageFormat::RgbaPremul,
            image.clone().width() as usize,
            image.clone().height() as usize,
        );
        self.screen_fatto = true;
        self.flag_transparency = false;
    }

    pub fn do_screen_area(&mut self) {
        let screen = Screen::from_point(0, 0).unwrap();

        let image = screen
            .capture_area(
                ((self.area.start.x) * self.area.scale) as i32,
                ((self.area.start.y) * self.area.scale) as i32,
                (self.area.width) as u32,
                (self.area.heigth) as u32,
            )
            .unwrap();

        self.name = chrono::offset::Utc::now().to_string();
        self.name = self
            .name
            .replace(".", "-")
            .replace(":", "-")
            .replace(" ", "_");
        // self.name += &self.format.to_string();

        self.img = ImageBuf::from_raw(
            image.clone().into_raw(),
            druid::piet::ImageFormat::RgbaPremul,
            image.clone().width() as usize,
            image.clone().height() as usize,
        );

        self.screen_fatto = true;
        self.flag_transparency = false;
    }

    pub fn screen_window(&mut self, ctx: &mut EventCtx) {
        let window = WindowDesc::new(show_screen(ctx, self.img.clone(), self))
            .title(self.name.clone())
            .set_window_state(druid_shell::WindowState::Maximized)
            .set_always_on_top(true);
        ctx.new_window(window);
    }
}

pub fn show_screen(
    ctx: &mut EventCtx,
    image: ImageBuf,
    data: &mut Screenshot,
) -> impl Widget<Screenshot> {
    // println!("x:{},  y:{}", data.area.start.x, data.area.start.y);

    let img = Image::new(image.clone()).fill_mode(FillStrat::ScaleDown);

    // if data.area.width != 0.0 && data.area.heigth != 0.0 && data.flag_resize == false {
    //     img.set_clip_area(Some(Rect::new(
    //         data.area.start.x * data.area.scale,
    //         data.area.start.y * data.area.scale,
    //         (data.area.start.x * data.area.scale) + data.area.width,
    //         (data.area.start.y * data.area.scale) + data.area.heigth,
    //     )));
    // }

    let mut col = Flex::column();
    let mut row = Flex::row();
    // let mut row2 = Flex::row();

    let sizedbox = SizedBox::new(img).width(800.).height(500.);

    let resize_button =
        Button::new("resize").on_click(move |ctx: &mut EventCtx, data: &mut Screenshot, _env| {
            data.flag_resize = true;
        });

    let annulla_button =
        Button::new("cancel").on_click(move |ctx: &mut EventCtx, data: &mut Screenshot, _env| {
            data.flag_resize = false;
        });

    let copy_button = Button::new("copy to clipboard").on_click(
        move |ctx: &mut EventCtx, data: &mut Screenshot, _env| {
            let mut clip = Clipboard::new().unwrap();
            let formatted: ImageData = ImageData {
                width: data.img.width(),
                height: data.img.height(),
                bytes: Cow::from(data.img.raw_pixels()),
            };
            clip.set_image(formatted).unwrap();
            println!("Saved check your clipboard");
        },
    );

    let button1 = Either::new(
        |data: &Screenshot, _: &Env| data.flag_resize,
        annulla_button,
        copy_button,
    );

    let button2 = Either::new(
        |data: &Screenshot, _: &Env| data.flag_resize,
        Button::new("update"),
        resize_button,
    );

    row.add_child(button1);
    row.add_child(button2);
    col.add_child(row);

    // row2.add_child(sizedbox);
    col.add_default_spacer();
    col.add_default_spacer();

    col.add_child(
        ZStack::new(sizedbox).with_centered_child(Either::new(
            |data: &Screenshot, _: &Env| data.flag_resize,
            Painter::new(|ctx, data: &Screenshot, _env| {
                let rect = druid::Rect::from_points(
                    (data.resized_area.x, data.resized_area.y),
                    (data.resized_area.x + data.resized_area.width, data.resized_area.y + data.resized_area.height),
                );
                ctx.fill(rect, &Color::rgba(0.0, 0.0, 0.0, 0.5));
                ctx.stroke(rect, &druid::Color::RED, 2.0);
                
            })
            .center()
            .controller(ResizeController { selected_part: ResizeInteraction::NoInteraction }),
            druid::widget::Label::new(""),
        )),
    );

    // ZStack::new(sizedbox.center()).with_centered_child(
    //     Painter::new(|ctx: &mut PaintCtx<'_, '_, '_>, data: &Screenshot, _env| {
    //         let displays = screenshots::DisplayInfo::all().expect("error");
    //         let scale = displays[0].scale_factor as f64;
    //         let width = displays[0].width as f64 * scale;
    //         let height = displays[0].height as f64 * scale;
    //         // let (start, end) = (data.area.start, data.area.end);
    //         let rect = Rect::from_center_size(((width/2.) , (height/2.)), (data.area.width, data.area.heigth));

    //         ctx.fill(rect, &Color::rgba(0.0, 0.0, 0.0, 0.4));
    //         ctx.stroke(rect, &druid::Color::RED, 1.0);
    //     })
    //     .center(),
    // )

    col
}

pub fn draw_rect() -> impl Widget<Screenshot> {
    let paint = Painter::new(|ctx: &mut PaintCtx<'_, '_, '_>, data: &Screenshot, _env| {
        let (start, end) = (data.area.start, data.area.end);
        let rect = druid::Rect::from_points(start, end);
        ctx.fill(
            rect,
            &Color::rgba(
                data.area.rgba.r,
                data.area.rgba.g,
                data.area.rgba.b,
                data.area.rgba.a,
            ),
        );
        // ctx.stroke(rect, &druid::Color::RED, 0.8);
    })
    .controller(MouseClickDragController {
        t1: TimerToken::next(),
        flag: true,
    })
    .center();

    Flex::column().with_child(paint)
}

// pub fn draw_rect_resize() -> impl Widget<Screenshot> {
//     let paint = Painter::new(|ctx: &mut PaintCtx<'_, '_, '_>, data: &Screenshot, _env| {
//         // let (start, end) = (data.area.start, data.area.end);
//         let rect = Rect::from_points(
//             (0.0, 0.0),
//             (data.area.width, data.area.heigth)
//         );

//         ctx.fill(rect, &Color::rgba(0.0, 0.0, 0.0, 0.4));
//         ctx.stroke(rect, &druid::Color::RED, 1.0);
//     })
//     .center();

//     paint
// }
