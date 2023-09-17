use druid::{
    widget::{Button, FillStrat, Flex, Image, Painter, SizedBox},
    Rect, Color, Data, EventCtx, ImageBuf, Lens, PaintCtx, Point, RenderContext, Widget, WidgetExt,
    WindowDesc, WindowState, CursorDesc, TimerToken
};
// use druid_shell::{TimerToken};

use crate::controller::*;
use image::*;
use raster::{transform, Color as rasterColor, Image as rasterImage};
use screenshots::Screen;
use serde::{Deserialize, Serialize};

#[derive(Clone, Data, PartialEq, Debug, Serialize, Deserialize)]
pub enum Format {
    MainFormat,
    Png,
    Jpg,
    Gif,
}

impl Format {
    pub fn to_string(&self) -> String {
        match self {
            Format::MainFormat => "".to_string(),
            Format::Jpg => ".jpg".to_string(),
            Format::Png => ".png".to_string(),
            Format::Gif => ".gif".to_string(),
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct SelectedArea {
    pub start: Point,
    pub end: Point,
    pub width: f64,
    pub heigth: f64,
    pub scale: f64,
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
    pub area_transparency: f64,
    pub flag_selection: bool, //serve per fare far partire il controller solo dopo aver acquisito l'area
    pub full_screen: bool,
    pub time_interval: f64,
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
            area_transparency: 0.4,
            flag_selection: false,
            full_screen: false,
            time_interval: 0.0,
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

        self.format = Format::MainFormat; //default
        self.name = time;
        self.name = self
            .name
            .replace(".", "-")
            .replace(":", "-")
            .replace(" ", "_");
        self.name += &self.format.to_string();

        self.img = ImageBuf::from_raw(
            image.clone().into_raw(),
            druid::piet::ImageFormat::RgbaPremul,
            image.clone().width() as usize,
            image.clone().height() as usize,
        );
        self.screen_fatto = true;
        self.area_transparency = 0.4;
    }

    pub fn do_screen_area(&mut self) {
        // println!("{}", self.area_transparency);
        let screen = Screen::from_point(0, 0).unwrap();
        let image = screen
            .capture_area(
                ((self.area.start.x) * self.area.scale) as i32,
                ((self.area.start.y) * self.area.scale) as i32,
                (self.area.width) as u32,
                (self.area.heigth) as u32,
            )
            .unwrap();
        self.format = Format::MainFormat; //default
        self.name = chrono::offset::Utc::now().to_string();
        self.name = self
            .name
            .replace(".", "-")
            .replace(":", "-")
            .replace(" ", "_");
        self.name += &self.format.to_string();

        self.img = ImageBuf::from_raw(
            image.clone().into_raw(),
            druid::piet::ImageFormat::RgbaPremul,
            image.clone().width() as usize,
            image.clone().height() as usize,
        );

        self.screen_fatto = true;
        self.area_transparency = 0.4;
    }

    pub fn screen_window(&mut self, ctx: &mut EventCtx) {
        let window = WindowDesc::new(show_screen(self.img.clone()))
            .title(self.name.clone())
            .set_window_state(druid_shell::WindowState::Maximized)
            .set_always_on_top(true);
        ctx.new_window(window);
    }
}

pub fn show_screen(image: ImageBuf) -> impl Widget<Screenshot> {
    let mut img = Image::new(image).fill_mode(FillStrat::ScaleDown);
    // img.set_clip_area(
    //     Some(Rect::new(10.0, 10.0, 300.0, 300.0))
    // );

    // let (w, h) = (image.width(), image.height());

    // let mut img2 = rasterImage::blank(w as i32, h as i32);

    // let mut x = 0;
    // let mut y = 0;
    // let rows = image.pixel_colors();
    // for row in rows {
    //     for color in row {
    //         let (r, g, b, a) = color.as_rgba();
    //         let color2 = rasterColor::rgba(
    //             (r * 255.0) as u8,
    //             (g * 255.0) as u8,
    //             (b * 255.0) as u8,
    //             (a * 255.0) as u8,
    //         );
    //         img2.set_pixel(x, y, color2);
    //         x = (x + 1) % w as i32;
    //     }
    //     y = y + 1;
    // }


    let mut col = Flex::column();
    let mut row = Flex::row();
    let mut row2 = Flex::row();

    let mut sizedbox = SizedBox::new(img).width(1400.).height(750.);

    let ruota_button =
        Button::new("🔄").on_click(move |ctx: &mut EventCtx, _data: &mut Screenshot, _env| {
            // transform::rotate(img, 45, rasterColor::rgb(0, 0, 0));
        });
    let crop_button = Button::new("ritaglia").on_click(move |ctx: &mut EventCtx, _data: &mut Screenshot, _env| {
        // let mut current = ctx.window().clone();
        // current.set_window_state(WindowState::Minimized);

        // let new_win = WindowDesc::new(
        //     draw_rect()
        // )
        // .show_titlebar(false)
        // .transparent(true)
        // .window_size((1000., 1000.))
        // .resizable(false)
        // .set_position((0.0, 0.0));

        // ctx.new_window(new_win);
    });
    
    row.add_child(ruota_button);
    row.add_child(crop_button);
    col.add_child(row);
    row2.add_child(sizedbox);
    col.add_child(row2);
    col
}


pub fn draw_rect() -> impl Widget<Screenshot> {
    let paint = Painter::new(|ctx: &mut PaintCtx<'_, '_, '_>, data: &Screenshot, _env| {
        let (start, end) = (data.area.start, data.area.end);
        let rect = druid::Rect::from_points(start, end);
        ctx.fill(rect, &Color::rgba(0.0, 0.0, 0.0, data.area_transparency));
        // ctx.stroke(rect, &druid::Color::RED, 0.8);
    })
    .controller(MouseClickDragController {t1: TimerToken::next(), flag: true })
    .center();

    Flex::column().with_child(paint)
}