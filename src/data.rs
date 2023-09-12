use druid::{
    widget::{FillStrat, Flex, Image, SizedBox},
    Data, ImageBuf, Lens,
};
use druid::{EventCtx, Widget, WindowDesc, MouseEvent, MouseButton, Env};
use image::*;
use screenshots::{Screen};
use serde::{Deserialize, Serialize};

use self::screenshot_derived_lenses::new_name;

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
pub struct SelectedArea{
    selecting_region: bool,
    start_x: f64,
    start_y: f64,
    end_x: f64,
    end_y: f64,
}
impl SelectedArea{
    pub fn new()->Self{
        Self { selecting_region: false, start_x: 0.0, start_y: 0.0, end_x: 0.0, end_y: 0.0 }
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
        }
    }

    pub fn toggle_textbox_state(data: &mut Screenshot) {
        if data.editing_name {
            data.editing_name = false;
        } else {
            data.editing_name = true;
        }
    }

    pub fn do_screen(&mut self, ctx: &mut EventCtx) {
        // let start = Instant::now();
        let screens = Screen::all().unwrap();

        // for screen in screens {
        // println!("capturer {screen:?}");
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

        // let window = WindowDesc::new(show_screen(self.img.clone()));
        // ctx.new_window(window);

        // image
        //     .save(format!("target/screens/{}.png", data.name))
        //     .unwrap();

        // }

        // let screen = Screen::from_point(100, 100).unwrap();
        // println!("capturer {screen:?}");

        // let image = screen.capture_area(300, 300, 300, 300).unwrap();
        // image.save("target/capture_display_with_point.png").unwrap();
        // println!("tempo di esecuzione: {:?}", start.elapsed());
    }

    pub fn screen_window(&mut self, ctx: &mut EventCtx) {
        let window = WindowDesc::new(show_screen(self.img.clone())).title(self.name.clone());
        ctx.new_window(window);
    }
}

pub fn show_screen(image: ImageBuf) -> impl Widget<Screenshot> {
    let img = Image::new(image).fill_mode(FillStrat::ScaleDown);
    SizedBox::new(img).width(1000.).height(1000.)
}


//capture area
fn on_mouse_down(_ctx: &mut EventCtx, data: &mut Screenshot, event: &MouseEvent, _env: &Env) {
    if event.button == MouseButton::Left {
        data.area.selecting_region = true;
        data.area.start_x = event.pos.x;
        data.area.start_y = event.pos.y;
    }
}

fn on_mouse_up(_ctx: &mut EventCtx, data: &mut Screenshot, event: &MouseEvent, _env: &Env) {
    if event.button == MouseButton::Left {
        data.area.selecting_region = false;
        data.area.end_x = event.pos.x;
        data.area.end_y = event.pos.y;
        // Cattura lo schermo nella regione selezionata
        capture_screen(data);
    }
}

fn capture_screen(data: &Screenshot) {
    if data.area.selecting_region {
        // Calcola le coordinate e le dimensioni della regione da catturare
        // let x1 = data.start_x.min(data.end_x);
        // let x2 = data.start_x.max(data.end_x);
        // let y1 = data.start_y.min(data.end_y);
        // let y2 = data.start_y.max(data.end_y);
        let displays = screenshots::DisplayInfo::all().expect("error");
        let display = displays[0];
        let screen = screenshots::Screen::new(&display);

        // Specifica le coordinate della regione da catturare (x, y, larghezza, altezza)
        let region = screen.capture_area(data.area.start_x as i32, data.area.start_y as i32, (data.area.end_x-data.area.start_x) as u32, (data.area.end_y-data.area.start_y) as u32).unwrap();
        
        // self.format = Format::MainFormat; //default
        // self.name = time;
        // self.name = self
        //     .name
        //     .replace(".", "-")
        //     .replace(":", "-")
        //     .replace(" ", "_");
        // self.name += &self.format.to_string();

        // self.img = ImageBuf::from_raw(
        //     image.clone().into_raw(),
        //     druid::piet::ImageFormat::RgbaPremul,
        //     image.clone().width() as usize,
        //     image.clone().height() as usize,
        // );

        // self.screen_fatto = true;

        // // Cattura la regione dello schermo specificata
        // let screenshot = capture_region(&region).expect("Failed to capture screen");

        // // Converti la cattura in un'immagine DynamicImage
        // let image = DynamicImage::ImageRgba8(ImageBuffer::from_raw(
        //     screenshot.width() as u32,
        //     screenshot.height() as u32,
        //     screenshot.into_vec(),
        // ).expect("Failed to create ImageBuffer"));

        // Salva l'immagine in un file (puoi anche mostrarla nell'app)
        // image.save("screenshot.png", ImageFormat::PNG).expect("Failed to save image");
    }
}