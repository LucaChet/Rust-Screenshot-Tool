use druid::{
    widget::{Button, FillStrat, Flex, Image, Painter, SizedBox},
    Color, Data, EventCtx, ImageBuf, Lens, PaintCtx, Point, RenderContext, Widget, WidgetExt,
    WindowDesc,
};

use crate::controller::*;
use image::*;
use kurbo::Affine;
use raster::{transform, Color as rasterColor, Image as rasterImage};
use screenshots::Screen;
use serde::{Deserialize, Serialize};
use nalgebra::Matrix3;

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
    pub window_minimized: bool,
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
            window_minimized: false,
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
            .set_window_state(druid_shell::WindowState::Maximized);
        ctx.new_window(window);
    }
}

pub fn show_screen(image: ImageBuf) -> impl Widget<Screenshot> {
    let img = Image::new(image).fill_mode(FillStrat::ScaleDown);

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

    let ruota_button =
        Button::new("🔄").on_click(move |ctx: &mut EventCtx, _data: &mut Screenshot, _env| {
            // transform::rotate(img, 45, rasterColor::rgb(0, 0, 0));
        });
    let crop_button = Button::new("ritaglia");

    row.add_child(ruota_button);
    row.add_child(crop_button);
    col.add_child(row);
    row2.add_child(SizedBox::new(img).width(900.).height(900.));
    col.add_child(row2);
    col
}

// Funzione per ruotare un ImageBuf di Druid utilizzando trasformazioni matriciali
fn rotate_image(image_buf: &ImageBuf, angle_degrees: f64) -> ImageBuf {
    let width = image_buf.width();
    let height = image_buf.height();

    // Calcola il centro dell'immagine
    let center_x = width as f64 / 2.0;
    let center_y = height as f64 / 2.0;

    // Converti l'angolo da gradi a radianti
    let angle_radians = angle_degrees.to_radians();

    // Crea una matrice di trasformazione per la rotazione
    let rotation_matrix = Matrix3::new_rotation(angle_radians);

    // Crea un nuovo ImageBuf con le stesse dimensioni
    let mut rotated_image_buf = ImageBuf::new(width, height);

    // Itera su ogni pixel nell'immagine originale
    for x in 0..width {
        for y in 0..height {
            // Calcola le coordinate del pixel nell'immagine ruotata
            let (new_x, new_y) = rotate_point(x as f64, y as f64, center_x, center_y, &rotation_matrix);

            // Copia il colore del pixel originale nell'immagine ruotata
            if let Some(color) = image_buf.get_pixel(new_x as usize, new_y as usize) {
                rotated_image_buf.set_pixel(x as usize, y as usize, color);
            }
        }
    }

    rotated_image_buf
}

// Funzione per ruotare un punto utilizzando una matrice di trasformazione
fn rotate_point(x: f64, y: f64, center_x: f64, center_y: f64, matrix: &Matrix3<f64>) -> (f64, f64) {
    let mut point = Matrix3::new_translation(-center_x, -center_y) * matrix * Matrix3::new_translation(center_x, center_y)
        * Matrix3::new_translation(x, y);

    point.x /= point.z;
    point.y /= point.z;

    (point.x, point.y)
}

pub fn draw_rect() -> impl Widget<Screenshot> {
    let paint = Painter::new(|ctx: &mut PaintCtx<'_, '_, '_>, data: &Screenshot, _env| {
        let (start, end) = (data.area.start, data.area.end);
        let rect = druid::Rect::from_points(start, end);
        ctx.fill(rect, &Color::rgba(0.0, 0.0, 0.0, data.area_transparency));
        // ctx.stroke(rect, &druid::Color::RED, 0.8);
    })
    .controller(MouseClickDragController {})
    .controller(ScreenArea {})
    .center();

    Flex::column().with_child(paint)
}

pub fn empty_window() -> impl Widget<Screenshot> {
    let paint = Painter::new(|ctx: &mut PaintCtx<'_, '_, '_>, data: &Screenshot, _env| {
        let (start, end) = (data.area.start, data.area.end);
        let rect = druid::Rect::from_points(start, end);
        ctx.fill(rect, &Color::rgba(0.0, 0.0, 0.0, data.area_transparency));
    })
    .controller(SetScreen {})
    .controller(ScreenArea {})
    .center();

    Flex::column().with_child(paint)
}
