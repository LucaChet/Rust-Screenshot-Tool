use druid::{
    widget::{
        Button, Container, Controller, Either, FillStrat, Flex, Image, Painter, SizedBox, ZStack, List,
    },
    Code, SysMods, HotKey, BoxConstraints, Color, CursorDesc, Data, Env, Event, EventCtx, ImageBuf, LayoutCtx, Lens,
    LifeCycle, LifeCycleCtx, PaintCtx, Point, Rect, RenderContext, Size, TimerToken, UpdateCtx,
    Widget, WidgetExt, WidgetPod, WindowDesc, WindowState,
};
use im::{Vector, HashMap};
use winit::monitor::MonitorHandle;
// use druid_shell::{TimerToken};

use crate::controller::*;
use arboard::Clipboard;
use arboard::ImageData;
use image::{codecs::png::PngDecoder, *};
use raster::{transform, Color as rasterColor, Image as rasterImage};
use screenshots::Screen;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, hash::Hash};

use self::selected_area_derived_lenses::heigth;

#[derive(Clone, Data, PartialEq, Debug, Serialize, Deserialize)]
pub enum Format {
    Png,
    Jpg,
    Gif,
    Pnm,
    Tga,
    Qoi,
    Tiff,
    Webp,
    Bmp
}

impl Format {
    pub fn to_string(&self) -> String {
        match self {
            Format::Jpg => ".jpg".to_string(),
            Format::Png => ".png".to_string(),
            Format::Gif => ".gif".to_string(),
            Format::Pnm => ".pnm".to_string(),
            Format::Tga => ".tga".to_string(),
            Format::Qoi => ".qoi".to_string(),
            Format::Tiff => ".tiff".to_string(),
            Format::Webp => ".webp".to_string(),
            Format::Bmp => ".bmp".to_string(),
        }
    }
}

#[derive(Clone, Data, PartialEq, Eq, Debug, Serialize, Deserialize, Hash)]
pub enum Shortcut{
    Save,
    SaveAs,
    Open,
    Customize,
    Screenshot,
    Capture,
    Quit,
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
pub struct ResizedArea {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
impl ResizedArea {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }
    pub fn new_parameter(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
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
    pub shortcut: HashMap<Shortcut, String>,
    pub selected_shortcut: Shortcut,
    pub editing_shortcut: bool,
    pub duplicate_shortcut: bool,
    pub monitor_id: usize,
}

impl Screenshot {
    pub fn new(name: String, format: Format, newname: String) -> Self {
        let mut shortcut = HashMap::new();
        shortcut.insert(Shortcut::Save, String::from("s"));
        shortcut.insert(Shortcut::SaveAs, String::from("a"));
        shortcut.insert(Shortcut::Open, String::from("o"));
        shortcut.insert(Shortcut::Customize, String::from("k"));
        shortcut.insert(Shortcut::Screenshot, String::from("t"));
        shortcut.insert(Shortcut::Capture, String::from("y"));
        shortcut.insert(Shortcut::Quit, String::from("q"));

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
            shortcut,
            selected_shortcut: Shortcut::Save,
            editing_shortcut: true,
            duplicate_shortcut: false,
            monitor_id: 0,
        }
    }

    pub fn toggle_textbox_state(data: &mut Screenshot) {
        if data.editing_name {
            data.editing_name = false;
        } else {
            data.editing_name = true;
        }
    }

    pub fn action_screen(&mut self, ctx: &mut EventCtx){
        let displays = screenshots::DisplayInfo::all().expect("error");
        let scale = displays[0].scale_factor as f64;
        let width = displays[0].width as f64 * scale;
        let height = displays[0].height as f64 * scale;

        let mut current = ctx.window().clone();
        current.set_window_state(WindowState::Minimized);
        self.full_screen = true;

        self.area.start = Point::new(0.0, 0.0);
        self.area.end = Point::new(0.0, 0.0);
        self.area.width = 0.0;
        self.area.heigth = 0.0;
        self.area.rgba.reset();

        let new_win = WindowDesc::new(draw_rect())
            .show_titlebar(false)
            .transparent(true)
            .window_size((width, height))
            // .set_window_state(WindowState::Maximized)
            .resizable(true)
            .set_position((0.0, 0.0))
            .set_always_on_top(true);

        ctx.new_window(new_win);
    }

    pub fn action_capture(&mut self, ctx: &mut EventCtx){
        let displays = screenshots::DisplayInfo::all().expect("error");
        let scale = displays[0].scale_factor as f64;
        let width = displays[0].width as f64 * scale;
        let height = displays[0].height as f64 * scale;

        let mut current = ctx.window().clone();
        current.set_window_state(WindowState::Minimized);
        self.full_screen = false;

        self.area.start = Point::new(0.0, 0.0);
        self.area.end = Point::new(0.0, 0.0);
        self.area.width = 0.0;
        self.area.heigth = 0.0;
        self.area.rgba.reset();

        let container = Either::new(
            |data: &Screenshot, _: &Env| data.flag_transparency,
            Container::new(draw_rect()).background(Color::rgba(0.0, 0.0, 0.0, 0.0)),
            Container::new(draw_rect()).background(Color::rgba(0.0, 0.0, 0.0, 0.6)),
        );

        let container2 = Either::new(
            |data: &Screenshot, _: &Env| data.flag_transparency,
            Container::new(draw_rect()).background(Color::rgba(0.0, 0.0, 0.0, 0.0)),
            Container::new(draw_rect()).background(Color::rgba(0.0, 0.0, 0.0, 0.6)),
        );

        let stack = Either::new(
            |data: &Screenshot, _: &Env| data.monitor_id == 0,
            container,
            {                
                let img = Image::new(self.img.clone());
                let sizedbox = SizedBox::new(Flex::column().with_child(img)).fix_size(width, height);
                let col = ZStack::new(sizedbox)
                .with_centered_child(container2);
                col
            }
        ).center();

        let new_win = WindowDesc::new(stack)
            .show_titlebar(false)
            .transparent(true)
            .window_size((width, height))
            .resizable(false);
            // .set_window_state(WindowState::Maximized);
            // .set_position((0.0, 0.0));

        ctx.new_window(new_win);
    }

    pub fn do_screen(&mut self) {
        let screens = Screen::all().unwrap();
        let image: ImageBuffer<Rgba<u8>, Vec<u8>> = screens[self.monitor_id].capture().unwrap();
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
        let screens = Screen::all().unwrap();
        let image = screens[0]
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

    pub fn reset_resize_rect(&mut self) {
        let area_width = 800.;
        let area_height = 500.;
        let original_width = self.img.width() as f64;
        let original_height = self.img.height() as f64;

        // Calcola le dimensioni ridimensionate dell'immagine mantenendo i rapporti tra larghezza e altezza.
        let mut new_width = original_width;
        let mut new_height = original_height;

        if original_width > area_width {
            new_width = area_width;
            new_height = (area_width * original_height) / original_width;
        }

        if new_height > area_height {
            new_height = area_height;
            new_width = (area_height * original_width) / original_height;
        }

        let center_x = area_width / 2.;
        let center_y = area_height / 2.;

        let top_left_x = center_x - (new_width / 2.);
        let top_left_y = center_y - (new_height / 2.);

        self.resized_area.x = top_left_x;
        self.resized_area.y = top_left_y;
        self.resized_area.width = new_width;
        self.resized_area.height = new_height;
    }

}

pub fn show_screen(
    ctx: &mut EventCtx,
    image: ImageBuf,
    data: &mut Screenshot,
) -> impl Widget<Screenshot> {
    data.flag_resize = false;
    data.reset_resize_rect();
    let original_x = data.resized_area.x;
    let original_y = data.resized_area.y;
    let original_w = data.resized_area.width;
    let original_h = data.resized_area.height;

    let img = Image::new(image.clone()).fill_mode(FillStrat::ScaleDown);

    let mut col = Flex::column();
    let mut row = Flex::row();

    let sizedbox = SizedBox::new(img).width(800.).height(500.);

    let resize_button =
        Button::new("resize").on_click(move |ctx: &mut EventCtx, data: &mut Screenshot, _env| {
            data.flag_resize = true;
        });

    let annulla_button =
        Button::new("cancel").on_click(move |ctx: &mut EventCtx, data: &mut Screenshot, _env| {
            data.flag_resize = false;
            data.reset_resize_rect();
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

    let update_button =
        Button::new("update").on_click(move |ctx: &mut EventCtx, data: &mut Screenshot, _env| {
            let image1: ImageBuffer<image::Rgba<u8>, Vec<u8>> = ImageBuffer::from_vec(
                data.img.width() as u32,
                data.img.height() as u32,
                data.img.raw_pixels().to_vec(),
            )
            .unwrap();

            let dynamic_image = DynamicImage::ImageRgba8(image1);
            let new = dynamic_image.crop_imm(
                ((data.resized_area.x - original_x) * (data.img.width() as f64 / original_w))
                    as u32,
                ((data.resized_area.y - original_y) * (data.img.height() as f64 / original_h))
                    as u32,
                (data.resized_area.width * (data.img.width() as f64 / original_w)) as u32,
                (data.resized_area.height * (data.img.height() as f64 / original_h)) as u32,
            );
            let image2 = new.to_rgba8();

            data.img = ImageBuf::from_raw(
                image2.clone().into_raw(),
                druid::piet::ImageFormat::RgbaPremul,
                image2.clone().width() as usize,
                image2.clone().height() as usize,
            );

            data.screen_window(ctx);
            ctx.window().close();
        });

    let button1 = Either::new(
        |data: &Screenshot, _: &Env| data.flag_resize,
        annulla_button,
        copy_button,
    );

    let button2 = Either::new(
        |data: &Screenshot, _: &Env| data.flag_resize,
        update_button,
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
                    (
                        data.resized_area.x + data.resized_area.width,
                        data.resized_area.y + data.resized_area.height,
                    ),
                );
                ctx.fill(rect, &Color::rgba(0.0, 0.0, 0.0, 0.5));
                ctx.stroke(rect, &druid::Color::RED, 2.0);
            })
            .center()
            .controller(ResizeController {
                selected_part: ResizeInteraction::NoInteraction,
                original_area: ResizedArea::new_parameter(
                    data.resized_area.x,
                    data.resized_area.y,
                    data.resized_area.width,
                    data.resized_area.height,
                ),
            }),
            druid::widget::Label::new(""),
        )),
    );
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
        flag_desk2: false,
    })
    .center();

    // Flex::column().with_child(paint)
    paint
}
