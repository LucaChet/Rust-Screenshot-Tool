use druid::{
    widget::{Button, Container, Controller, FillStrat, Flex, Image, Painter, SizedBox},
    BoxConstraints, Color, CursorDesc, Data, Env, Event, EventCtx, ImageBuf, LayoutCtx, Lens,
    LifeCycle, LifeCycleCtx, PaintCtx, Point, Rect, RenderContext, Size, TimerToken, UpdateCtx,
    Widget, WidgetExt, WidgetPod, WindowDesc, WindowState,
};
// use druid_shell::{TimerToken};

use crate::controller::*;
use arboard::Clipboard;
use arboard::ImageData;
use std::borrow::Cow;
use image::{*, codecs::png::PngDecoder};
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
    pub flag_resize: bool, //usato per mostrare screen in show_screen durante il resize
    pub rect: Rect,
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
            flag_resize: false,
            rect: Rect::new(100.0, 100.0, 200.0, 200.0),
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

    let mut img = Image::new(image.clone()).fill_mode(FillStrat::ScaleDown);
    // let copia_img = Image::new(image.clone()).fill_mode(FillStrat::ScaleDown);

    let mut col = Flex::column();

    if data.area.width != 0.0 && data.area.heigth != 0.0 && data.flag_resize == false {
        img.set_clip_area(Some(Rect::new(
            data.area.start.x * data.area.scale,
            data.area.start.y * data.area.scale,
            (data.area.start.x * data.area.scale) + data.area.width,
            (data.area.start.y * data.area.scale) + data.area.heigth,
        )));
    }
    // else if data.flag_resize {
    //     // let displays = screenshots::DisplayInfo::all().expect("error");
    //     // let scale = displays[0].scale_factor as f64;
    //     // let width = displays[0].width as f64 * scale;
    //     // let height = displays[0].height as f64 * scale;
    //     // let new_win = WindowDesc::new(draw_rect_resize())
    //     //     .show_titlebar(false)
    //     //     .transparent(true)
    //     //     .window_size((data.area.width, data.area.heigth))
    //     //     .resizable(true)
    //     //     .set_position((0.0, 0.0))
    //     //     .set_always_on_top(true);

    //     // ctx.new_window(new_win);
    //     col.add_child(draw_rect_resize());
    // }

    let mut row = Flex::row();
    let mut row2 = Flex::row();

    let sizedbox = SizedBox::new(img).width(1200.).height(700.);

    let resize_button =
        Button::new("resize").on_click(move |ctx: &mut EventCtx, data: &mut Screenshot, _env| {
            // data.flag_resize = true;
            // data.screen_window(ctx);
            // ctx.window().clone().close();

            let displays = screenshots::DisplayInfo::all().expect("error");
            let scale = displays[0].scale_factor as f64;
            let width = displays[0].width as f64 * scale;
            let height = displays[0].height as f64 * scale;
            let new_win = WindowDesc::new(draw_rect_resize(ctx, image.clone(), &mut data.clone()))
                .title("Image Crop")
                .window_size((width, height))
                .set_position((0.0, 0.0))
                .show_titlebar(true);

            ctx.new_window(new_win);
        });

    let copy_button =
        Button::new("copy to clipboard").on_click(move |ctx: &mut EventCtx, data: &mut Screenshot, _env| {
            let mut clip = Clipboard::new().unwrap();
            let formatted: ImageData = ImageData { width: data.img.width(), height: data.img.height(), bytes: Cow::from(data.img.raw_pixels()) };
            clip.set_image(formatted).unwrap();
            println!("Saved check your clipboard");
            let imm = clip.get_image().unwrap();
            let displays = screenshots::DisplayInfo::all().expect("error");
            let scale = displays[0].scale_factor as f64;
            let width = displays[0].width as f64 * scale;
            let height = displays[0].height as f64 * scale;
            let new_win = WindowDesc::new(draw_rect_resize(ctx, ImageBuf::from_raw(imm.bytes, druid::piet::ImageFormat::RgbaPremul, data.img.width(), data.img.height()), &mut data.clone()))
                .title("Image Crop")
                .window_size((width, height))
                .set_position((0.0, 0.0))
                .show_titlebar(true);

            ctx.new_window(new_win);
        });

    row.add_child(copy_button);
    row.add_child(resize_button);
    col.add_child(row);

    row2.add_child(sizedbox);
    col.add_child(row2);

    // if data.flag_resize{
    //     col.add_child(rect);
    // }

    col
}

pub fn draw_rect() -> impl Widget<Screenshot> {
    let paint = Painter::new(|ctx: &mut PaintCtx<'_, '_, '_>, data: &Screenshot, _env| {
        let (start, end) = (data.area.start, data.area.end);
        let rect = druid::Rect::from_points(start, end);
        ctx.fill(rect, &Color::rgba(0.0, 0.0, 0.0, data.area_transparency));
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
//         let rect = Rect::new(
//             data.area.start.x * data.area.scale,
//             data.area.start.y * data.area.scale,
//             (data.area.start.x * data.area.scale) + data.area.width,
//             (data.area.start.y * data.area.scale) + data.area.heigth,
//         );

//         ctx.fill(rect, &Color::rgba(100.0, 100.0, 100.0, 0.4));
//         ctx.stroke(rect, &druid::Color::RED, 1.0);
//     })
//     // .controller(MouseClickDragController {
//     //     t1: TimerToken::next(),
//     //     flag: true,
//     // })
//     .center();

//     // Flex::row().with_child(paint)
//     // WidgetPod::new(paint)
//     paint
//     // .fill_strategy(FillStrat::Fill)
// }

// Funzione per la finestra di ritaglio
fn draw_rect_resize(
    ctx: &mut EventCtx,
    image: ImageBuf,
    data: &mut Screenshot,
) -> impl Widget<Screenshot> {
    // Crea una widget tree con l'immagine e il rettangolo ridimensionabile
    let mut flex = Flex::row();

    // Crea l'immagine di sfondo
    let mut img = Image::new(image.clone()).fill_mode(FillStrat::ScaleDown);
    // Imposta il clip area per il rettangolo di ritaglio
    // img.set_clip_area(Some(Rect::ZERO)); // Puoi inizializzare con un rettangolo vuoto

    // Aggiungi l'immagine e il rettangolo alla widget tree
    flex.add_child(SizedBox::new(img).width(1000.).height(600.));
    // Aggiungi qui il rettangolo ridimensionabile, ad esempio utilizzando un widget personalizzato
    // let resizable_rect = ResizableRect::new(Rect::ZERO); // Inizializza con un rettangolo vuoto
    let rect = Rect::new(50.0, 50.0, 50.0, 50.0); // Crea un rettangolo con coordinate (50, 50) e dimensioni 200x150
    let resizable_rect = ResizableRect::new(rect);
    flex.add_child(resizable_rect);

    // Restituisci la widget tree
    flex
}

// Definisci un nuovo widget per il rettangolo ridimensionabile
pub struct ResizableRect {
    rect: Rect,
    resizing: bool,
}

impl ResizableRect {
    pub fn new(rect: Rect) -> Self {
        ResizableRect {
            rect,
            resizing: false,
        }
    }
}

impl Widget<Screenshot> for ResizableRect {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut Screenshot, _env: &Env) {
        match event {
            Event::MouseDown(mouse_event) if self.rect.contains(mouse_event.pos) => {
                self.resizing = true;
            }
            Event::MouseUp(_) => {
                self.resizing = false;
            }
            Event::MouseMove(mouse_event) if self.resizing => {
                // Ridimensiona il rettangolo in base al movimento del mouse
                self.rect = Rect::from_points(self.rect.origin(), mouse_event.pos);
                ctx.request_paint();
            }
            _ => (),
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &Screenshot, _env: &Env) {
        // Disegna il rettangolo
        ctx.fill(self.rect, &Color::rgba(0.0, 0.0, 1.0, 0.5));
        ctx.stroke(self.rect, &Color::rgb(0.0, 0.0, 0.0), 2.0);
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _ev: &LifeCycle,
        _data: &Screenshot,
        _env: &Env,
    ) {
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        _old_data: &Screenshot,
        _data: &Screenshot,
        _env: &Env,
    ) {
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        _bc: &BoxConstraints,
        _data: &Screenshot,
        _env: &Env,
    ) -> Size {
        Size::new(self.rect.width(), self.rect.height())
    }
}
