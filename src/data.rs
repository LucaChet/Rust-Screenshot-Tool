use druid::{
    widget::{Controller, FillStrat, Flex, Image, Label, Painter, SizedBox},
    Code, Color, Data, Env, Event, EventCtx, ImageBuf, Lens, MouseButton, PaintCtx, Point,
    RenderContext, Widget, WidgetExt, WindowDesc, WindowState, Cursor, AppDelegate, Handled, Target, commands, Command, DelegateCtx, FileSpec
};

use image::*;
use screenshots::Screen;
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;

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
pub struct SelectedArea {
    start: Point,
    end: Point,
    width: f64,
    heigth: f64,
    scale: f64,
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
    pub flag_selection: bool,
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
        let mut current = ctx.window().clone();
        current.set_window_state(WindowState::Minimized);
        // let state = current.get_window_state();
        
        let screens = Screen::all().unwrap();
        let image: ImageBuffer<Rgba<u8>, Vec<u8>> = screens[0].capture().unwrap();
        let time: String = chrono::offset::Utc::now().to_string();

        // set_window_state(WindowState::Minimized);

        // let duration = Duration::from_secs(2);
        // thread::sleep(duration);

        // let screens = Screen::all().unwrap();

        // for screen in screens {
        // println!("capturer {screen:?}");

        // current.set_window_state(WindowState::Restored);
        // current.show();

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

    }

    pub fn do_screen_area(&mut self){
        println!("{}", self.area_transparency);
        let screen = Screen::from_point(0, 0).unwrap();
        let image = screen
            .capture_area(
                ( (self.area.start.x) * self.area.scale) as i32,
                ( (self.area.start.y) * self.area.scale) as i32,
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
        self.area_transparency=0.4;

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
// fn on_mouse_down(_ctx: &mut EventCtx, data: &mut Screenshot, event: &MouseEvent, _env: &Env) {
//     if event.button == MouseButton::Left {
//         data.area.selecting_region = true;
//         data.area.start_x = event.pos.x;
//         data.area.start_y = event.pos.y;
//     }
// }

// fn on_mouse_up(_ctx: &mut EventCtx, data: &mut Screenshot, event: &MouseEvent, _env: &Env) {
//     if event.button == MouseButton::Left {
//         data.area.selecting_region = false;
//         data.area.end_x = event.pos.x;
//         data.area.end_y = event.pos.y;
//         // Cattura lo schermo nella regione selezionata
//         capture_screen(data);
//     }
// }

// fn capture_screen(data: &Screenshot) {
//     if data.area.selecting_region {
//         // Calcola le coordinate e le dimensioni della regione da catturare
//         // let x1 = data.start_x.min(data.end_x);
//         // let x2 = data.start_x.max(data.end_x);
//         // let y1 = data.start_y.min(data.end_y);
//         // let y2 = data.start_y.max(data.end_y);
//         let displays = screenshots::DisplayInfo::all().expect("error");
//         let display = displays[0];
//         let screen = screenshots::Screen::new(&display);

//         // Specifica le coordinate della regione da catturare (x, y, larghezza, altezza)
//         let region = screen.capture_area(data.area.start_x as i32, data.area.start_y as i32, (data.area.end_x-data.area.start_x) as u32, (data.area.end_y-data.area.start_y) as u32).unwrap();

//         // self.format = Format::MainFormat; //default
//         // self.name = time;
//         // self.name = self
//         //     .name
//         //     .replace(".", "-")
//         //     .replace(":", "-")
//         //     .replace(" ", "_");
//         // self.name += &self.format.to_string();

//         // self.img = ImageBuf::from_raw(
//         //     image.clone().into_raw(),
//         //     druid::piet::ImageFormat::RgbaPremul,
//         //     image.clone().width() as usize,
//         //     image.clone().height() as usize,
//         // );

//         // self.screen_fatto = true;

//         // // Cattura la regione dello schermo specificata
//         // let screenshot = capture_region(&region).expect("Failed to capture screen");

//         // // Converti la cattura in un'immagine DynamicImage
//         // let image = DynamicImage::ImageRgba8(ImageBuffer::from_raw(
//         //     screenshot.width() as u32,
//         //     screenshot.height() as u32,
//         //     screenshot.into_vec(),
//         // ).expect("Failed to create ImageBuffer"));

//         // Salva l'immagine in un file (puoi anche mostrarla nell'app)
//         // image.save("screenshot.png", ImageFormat::PNG).expect("Failed to save image");
//     }
// }


pub struct Delegate;


impl AppDelegate<Screenshot> for Delegate { 
    fn command( 
        &mut self, 
        _ctx: &mut DelegateCtx, 
        _target: Target, 
        cmd: &Command, 
        data: &mut Screenshot, 
        _env: &Env, 
    ) -> Handled { 
        if let Some(file_info) = cmd.get(commands::SAVE_FILE_AS) { 
            // let img_bytes: &[u8] = data.img.raw_pixels();
            // if let Err(e) = std::fs::write(file_info.path(), img_bytes) { 
            //     println!("Error writing file: {e}"); 
            // } 
                   // Specifica il formato dell'immagine (in questo caso PNG)
                   let color_type = ColorType::Rgba8;
                   let file = std::fs::File::create(file_info.path()).unwrap();
                   let encoder = image::codecs::png::PngEncoder::new(file);
       
                   if let Err(e) = encoder.write_image(data.img.raw_pixels(), data.img.width() as u32, data.img.height() as u32, color_type) {
                       println!("Error writing file: {}", e);
                   }
            return Handled::Yes; 
        } 
        Handled::No 
    } 
}

pub struct Enter;

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

pub struct MouseClickDragController;

impl<W: Widget<Screenshot>> Controller<Screenshot, W> for MouseClickDragController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut Screenshot,
        env: &Env,
    ) {
        match event {
            Event::MouseDown(mouse_event) => {
                if mouse_event.button == MouseButton::Left {
                    // Esegui qualcosa quando viene premuto il pulsante sinistro del mouse.
                    // Ad esempio, puoi iniziare a trascinare un elemento.
                    // Inizia a tenere traccia del punto in cui è iniziato il trascinamento.
                    
                    ctx.set_cursor(&Cursor::Crosshair);
                    let start_point = mouse_event.pos;
                    
                    ctx.set_active(true);
                    // ctx.set_handled();
                    
                    // Memorizza il punto iniziale nel data del widget o in un altro stato.
                    data.area.start = start_point;
                    data.area.end = start_point;
                }
            }
            Event::MouseUp(mouse_event) => {
                if mouse_event.button == MouseButton::Left && ctx.is_active() {
                    // Esegui qualcosa quando viene rilasciato il pulsante sinistro del mouse.
                    // Ad esempio, puoi terminare il trascinamento.

                    data.area_transparency = 0.0;
                    data.flag_selection = true;
                
                    ctx.set_active(false);
                    // ctx.set_handled();

                    // Calcola il punto finale del trascinamento e fai qualcosa con esso.
                    let end_point = mouse_event.pos;
                    data.area.end = end_point;

                    if mouse_event.pos.x < data.area.start.x {
                        data.area.start.x = mouse_event.pos.x;
                    }
                    if mouse_event.pos.y < data.area.start.y {
                        data.area.start.y = mouse_event.pos.y;
                    }

                    ctx.set_cursor(&Cursor::Arrow);                
                }
            }
            Event::MouseMove(mouse_event) => {
                if ctx.is_active() {
                    // Esegui qualcosa quando il mouse viene spostato durante il trascinamento.
                    // Ad esempio, aggiorna la posizione dell'elemento trascinato.
                    let end_point = mouse_event.pos;
                    data.area.end = end_point;
                    // Calcola la differenza tra la posizione attuale e quella iniziale.

                    let deltax = (mouse_event.pos.x - data.area.start.x).abs() * data.area.scale;
                    let deltay = (mouse_event.pos.y - data.area.start.y).abs() * data.area.scale;
                    
                    data.area.width = (deltax).abs();
                    data.area.heigth = (deltay).abs();

                    // ctx.request_paint();
                }
            }
            _ => {}
        }

        child.event(ctx, event, data, env);
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


pub struct ScreenArea;

impl<W: Widget<Screenshot>> Controller<Screenshot, W> for ScreenArea {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut Screenshot,
        env: &Env,
    ) {
        if data.flag_selection && data.area_transparency==0.0 && !ctx.is_active(){
            if data.area.width != 0.0 && data.area.heigth != 0.0{
                data.do_screen_area();
                // data.area_transparency = 0.4;
            }
            data.area.start = Point::new(0.0, 0.0);
            data.area.end = Point::new(0.0, 0.0);
            data.flag_selection = false;
            ctx.window().close();
        }
        child.event(ctx, event, data, env);
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

// pub fn empty_window() -> impl Widget<Screenshot> {
//     let paint = Painter::new(|ctx: &mut PaintCtx<'_, '_, '_>, data: &Screenshot, _env| {
//         // let (start, end) = (data.area.start, data.area.end);
//         // let rect = druid::Rect::from_points(start, end);
//         // ctx.fill(rect, &Color::rgba(0.0, 0.0, 0.0, 0.4));
//         //ctx.stroke(rect, &druid::Color::WHITE, 1.0);
//         // data.do_screen(ctx);
//     });
//     // .controller(MouseClickDragController {})
//     // .controller(SelectionScreenController{})
//     // .center();

//     Flex::column().with_child(Label::new(""))
// }
