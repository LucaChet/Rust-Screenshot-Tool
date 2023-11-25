use druid::{
    FileDialogOptions, Selector, WindowDesc, commands, AppDelegate, Code, Command, Cursor, DelegateCtx, Env, Event, EventCtx
    ,Handled, LocalizedString, MouseButton, Point, Target, Widget, Color,
    WindowState, CursorDesc, ImageBuf, FileSpec
};

use std::time::Duration;

use druid::widget::Controller;
use druid_shell::{TimerToken, Application};

use crate::ui::*;
use crate::data::*;
use image::*;
pub const SHORTCUT: Selector = Selector::new("shortcut_selector");

pub struct Enter;
//controller che gestisce la modica del nome alla pressione del tasto invio
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
                    data.flag_focus = true;
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

pub struct MouseClickDragController {
    pub t1: TimerToken,
    pub flag: bool,
}

impl<W: Widget<Screenshot>> Controller<Screenshot, W> for MouseClickDragController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &druid::Event,
        data: &mut Screenshot,
        env: &Env,
    ) {
        if data.full_screen == false {
            let mut current = ctx.window().clone();

            if data.time_interval > 0.0 && self.flag && !data.flag_desk2{ //flag_desk2 serve per il secondo monitor, scatta dopo tot secondi e al secondo giro entra nell'else if
                self.t1 = ctx.request_timer(Duration::from_secs(data.time_interval as u64));
                self.flag = false;
                current.set_window_state(WindowState::Minimized);
            } else if self.flag {
                self.t1 = ctx.request_timer(Duration::from_millis(10));
                self.flag = false;
                current.set_window_state(WindowState::Minimized);
                ctx.set_cursor(&Cursor::Crosshair);
            }
            match event {
                Event::MouseDown(mouse_event) => {
                    if mouse_event.button == MouseButton::Left {
                        let start_point = mouse_event.pos;

                        ctx.set_active(true);

                        // Memorizza il punto iniziale
                        data.area.start = start_point;
                        data.area.end = start_point;
                    }
                }
                Event::MouseUp(mouse_event) => {
                    if mouse_event.button == MouseButton::Left && ctx.is_active() {
                        // Esegui qualcosa quando viene rilasciato il pulsante sinistro del mouse.
                        // Ad esempio, puoi terminare il trascinamento.

                        data.flag_transparency = true;

                        data.area.rgba.r = 0.0;
                        data.area.rgba.g = 0.0;
                        data.area.rgba.b = 0.0;
                        data.area.rgba.a = 0.0;
                        data.flag_selection = true;
                        data.flag_desk2 = false;
                        self.t1 = ctx.request_timer(Duration::from_millis(500));

                        ctx.set_active(false);

                        // Calcola il punto finale del trascinamento
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
                        let end_point = mouse_event.pos;
                        data.area.end = end_point;

                        let deltax =
                            (mouse_event.pos.x - data.area.start.x).abs() * data.area.scale;
                        let deltay =
                            (mouse_event.pos.y - data.area.start.y).abs() * data.area.scale;

                        data.area.width = (deltax).abs();
                        data.area.heigth = (deltay).abs();

                        // ctx.request_paint();
                    }
                }
                Event::Timer(id) => {
                    if self.t1 == *id && data.flag_selection {
                        if data.area.width != 0.0 && data.area.heigth != 0.0 {
                            data.do_screen_area(); //dovrebbe essere do_screen_area -> cambio per prova
                            self.flag = true;
                        }
                        data.flag_selection = false;
                        data.screen_window(ctx);
                        ctx.window().close();
                    } else if self.t1 == *id {
                        //posso selezionare dopo tot secondi
                        if data.monitor_id != 0 && !data.flag_desk2{
                            data.action_capture(ctx);
                            data.flag_desk2 = true;
                            ctx.window().close();
                        }else{
                            current.set_always_on_top(true);
                            current.set_window_state(WindowState::Restored);
                            ctx.set_cursor(&Cursor::Crosshair);
                        }
                    }
                }
                _ => {}
            }
        } else if data.full_screen {
            let mut current = ctx.window().clone();
            current.set_window_state(WindowState::Minimized);

            if data.time_interval < 0.5 && self.flag {
                self.t1 = ctx.request_timer(Duration::from_millis(500));
                self.flag = false;
            } else if self.flag {
                self.t1 = ctx.request_timer(Duration::from_secs(data.time_interval as u64));
                self.flag = false;
            }
            match event {
                Event::Timer(id) => {
                    if self.t1 == *id {
                        data.do_screen();
                        self.flag = true;
                        data.screen_window(ctx);
                        ctx.window().close();
                    }
                }

                _ => {}
            }
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

pub enum ResizeInteraction {
    NoInteraction,
    Area(f64, f64),
    Upper,
    Bottom,
    Left,
    Right,
}
pub struct ResizeController {
    pub selected_part: ResizeInteraction,
    pub original_area: ResizedArea,
}

impl<W: Widget<Screenshot>> Controller<Screenshot, W> for ResizeController {
    fn event(
        &mut self,
        _child: &mut W,
        ctx: &mut EventCtx,
        event: &druid::Event,
        data: &mut Screenshot,
        _env: &Env,
    ) {
        let delta = 3.0;
        match event {
            Event::MouseDown(mouse_event) => {
                ctx.set_active(true);
                // Controlla il bordo superiore.
                if mouse_event.pos.x >= data.resized_area.x
                    && mouse_event.pos.x <= data.resized_area.x + data.resized_area.width
                    && mouse_event.pos.y >= data.resized_area.y - delta
                    && mouse_event.pos.y < data.resized_area.y + delta
                {
                    self.selected_part = ResizeInteraction::Upper;
                }
                // Controlla il bordo inferiore.
                else if mouse_event.pos.x >= data.resized_area.x
                    && mouse_event.pos.x <= data.resized_area.x + data.resized_area.width
                    && mouse_event.pos.y >= data.resized_area.y + data.resized_area.height - delta
                    && mouse_event.pos.y <= data.resized_area.y + data.resized_area.height
                {
                    self.selected_part = ResizeInteraction::Bottom;
                }
                // Controlla il bordo destro.
                else if mouse_event.pos.x >= data.resized_area.x + data.resized_area.width - delta
                    && mouse_event.pos.x <= data.resized_area.x + data.resized_area.width
                    && mouse_event.pos.y >= data.resized_area.y
                    && mouse_event.pos.y <= data.resized_area.y + data.resized_area.height
                {
                    self.selected_part = ResizeInteraction::Right;
                }
                // Controlla il bordo sinistro.
                else if mouse_event.pos.x >= data.resized_area.x
                    && mouse_event.pos.x < data.resized_area.x + delta
                    && mouse_event.pos.y >= data.resized_area.y
                    && mouse_event.pos.y <= data.resized_area.y + data.resized_area.height
                {
                    self.selected_part = ResizeInteraction::Left;
                }
                // Controlla l'interno dell'area.
                else if mouse_event.pos.x > data.resized_area.x
                    && mouse_event.pos.x < data.resized_area.x + data.resized_area.width
                    && mouse_event.pos.y > data.resized_area.y
                    && mouse_event.pos.y < data.resized_area.y + data.resized_area.height
                {
                    self.selected_part =
                        ResizeInteraction::Area(mouse_event.pos.x, mouse_event.pos.y);
                } else {
                    ctx.set_cursor(&Cursor::Arrow);
                }
            }
            Event::MouseUp(_mouse_event) => {
                ctx.request_paint();
                self.selected_part = ResizeInteraction::NoInteraction;
                ctx.set_active(false);
            }
            Event::MouseMove(mouse_event) => {
                // Controlla il bordo superiore.
                if mouse_event.pos.x >= data.resized_area.x
                    && mouse_event.pos.x <= data.resized_area.x + data.resized_area.width
                    && mouse_event.pos.y >= data.resized_area.y
                    && mouse_event.pos.y < data.resized_area.y + delta
                {
                    ctx.set_cursor(&Cursor::ResizeUpDown);
                }
                // Controlla il bordo inferiore.
                else if mouse_event.pos.x >= data.resized_area.x
                    && mouse_event.pos.x <= data.resized_area.x + data.resized_area.width
                    && mouse_event.pos.y >= data.resized_area.y + data.resized_area.height - delta
                    && mouse_event.pos.y <= data.resized_area.y + data.resized_area.height
                {
                    ctx.set_cursor(&Cursor::ResizeUpDown);
                }
                // Controlla il bordo destro.
                else if mouse_event.pos.x >= data.resized_area.x + data.resized_area.width - delta
                    && mouse_event.pos.x <= data.resized_area.x + data.resized_area.width
                    && mouse_event.pos.y >= data.resized_area.y
                    && mouse_event.pos.y <= data.resized_area.y + data.resized_area.height
                {
                    ctx.set_cursor(&Cursor::ResizeLeftRight);
                }
                // Controlla il bordo sinistro.
                else if mouse_event.pos.x >= data.resized_area.x
                    && mouse_event.pos.x < data.resized_area.x + delta
                    && mouse_event.pos.y >= data.resized_area.y
                    && mouse_event.pos.y <= data.resized_area.y + data.resized_area.height
                {
                    ctx.set_cursor(&Cursor::ResizeLeftRight);
                }
                // Controlla l'interno dell'area.
                else if mouse_event.pos.x > data.resized_area.x
                    && mouse_event.pos.x < data.resized_area.x + data.resized_area.width
                    && mouse_event.pos.y > data.resized_area.y
                    && mouse_event.pos.y < data.resized_area.y + data.resized_area.height
                {
                    ctx.set_cursor(&Cursor::Pointer);
                } else {
                    ctx.set_cursor(&Cursor::Arrow);
                }

                //update coordinates of the red rect
                let deltax = mouse_event.pos.x - data.resized_area.x;
                let deltay = mouse_event.pos.y - data.resized_area.y;

                if ctx.is_active() {
                    match self.selected_part {
                        ResizeInteraction::Area(start_x, start_y) => {
                            let deltax = mouse_event.pos.x - start_x;
                            let deltay = mouse_event.pos.y - start_y;
                            if data.resized_area.x + deltax >= self.original_area.x
                                && data.resized_area.x + data.resized_area.width + deltax
                                    <= self.original_area.x + self.original_area.width
                                && data.resized_area.y + deltay >= self.original_area.y
                                && data.resized_area.y + data.resized_area.height + deltay
                                    <= self.original_area.y + self.original_area.height
                            {
                                data.resized_area.x += deltax;
                                data.resized_area.y += deltay;
                            }
                            self.selected_part =
                                ResizeInteraction::Area(mouse_event.pos.x, mouse_event.pos.y);
                        }
                        ResizeInteraction::Bottom => {
                            let deltay = mouse_event.pos.y - (data.resized_area.y + data.resized_area.height);
                            if (data.resized_area.y + data.resized_area.height + deltay <= self.original_area.y + self.original_area.height)
                                && (data.resized_area.y + data.resized_area.height + deltay >= data.resized_area.y + 10.)
                            {
                                data.resized_area.height += deltay;
                            }
                        }
                        ResizeInteraction::Upper => {
                            if data.resized_area.y + deltay >= self.original_area.y &&
                            data.resized_area.y + deltay <= data.resized_area.y + data.resized_area.height - 10.{
                                data.resized_area.y += deltay;
                                data.resized_area.height -= deltay
                            }
                        }
                        ResizeInteraction::Left => {
                            if data.resized_area.x + deltax >= self.original_area.x &&
                            data.resized_area.x + deltax <= data.resized_area.x + data.resized_area.width - 10.{
                                data.resized_area.x += deltax;
                                data.resized_area.width -= deltax;
                            }
                        }
                        ResizeInteraction::Right => {
                            let deltax = mouse_event.pos.x - (data.resized_area.x + data.resized_area.width);
                            if data.resized_area.x + data.resized_area.width + deltax <= self.original_area.x + self.original_area.width &&
                            data.resized_area.x + data.resized_area.width + deltax >= data.resized_area.x + 10.
                            {
                                data.resized_area.width += deltax;
                            }
                        }
                        _ => (),
                    }
                }
            }
            _ => {}
        }
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

pub struct Delegate;

impl AppDelegate<Screenshot> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut Screenshot,
        _env: &Env,
    ) -> Handled {

        if let Some(file_info) = cmd.get(commands::SAVE_FILE_AS) {
            let color_type = ColorType::Rgba8;
            let file = std::fs::File::create(file_info.path()).unwrap();
            let encoder = image::codecs::png::PngEncoder::new(file);

            if let Err(e) = encoder.write_image(
                data.img.raw_pixels(),
                data.img.width() as u32,
                data.img.height() as u32,
                color_type,
            ) {
                println!("Error writing file: {}", e);
            }
            return Handled::Yes;
        }
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            match std::fs::read_dir(file_info.path()) {
                Ok(_) => {
                    data.default_save_path = String::from(file_info.path().to_str().unwrap());
                }
                Err(e) => {
                    println!("Error opening path: {e}");
                }
            }
            return Handled::Yes;
        }
        if cmd.is(SHORTCUT) {
            let new_win = WindowDesc::new(modify_shortcut())
                .title(LocalizedString::new("Shortcut"))
                .window_size((400.0, 300.0));
            ctx.new_window(new_win);
        }
        Handled::No
    }

}

pub struct HotKeyController{
    pub flag: bool, //serve per ordine tra modifiers e tasti normali
    pub first: bool,
    pub n_ctrl: usize,
    pub n_alt: usize,
    pub n_shift: usize,
}

impl<W: Widget<Screenshot>> Controller<Screenshot, W> for HotKeyController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut Screenshot,
        _env: &Env,
    ) {
        ctx.request_focus();

        if let Event::WindowDisconnected = event{
            data.saved_shortcut = false;
            data.shortcut_order = true;
            data.with_modifiers = false;
            data.one_key = true;
            data.dup_modifier = false;
            data.prec_hotkey = String::from("");
            data.new_shortcut = String::from("");
            data.duplicate_shortcut = false;

            self.flag = false;
            self.first = false;
            self.n_ctrl = 0;
            self.n_alt = 0;
            self.n_shift = 0;
        }

        if let Event::KeyDown(key) = event {

            if data.new_shortcut == "".to_string(){
                data.saved_shortcut = false;
                data.shortcut_order = true;
                data.with_modifiers = false;
                data.one_key = true;
                data.dup_modifier = false;

                self.flag = false;
                self.first = false;
                self.n_ctrl = 0;
                self.n_alt = 0;
                self.n_shift = 0;
            }
            data.duplicate_shortcut = false;

            let code = key.key.clone().to_string();
            let code2 = key.code.clone().to_string();

            if code != data.prec_hotkey{
                if data.prec_hotkey != "".to_string(){  //controllo su pressione prolungata
                    data.new_shortcut.push_str("+");
                }
                data.new_shortcut.push_str(code.as_str());
                data.prec_hotkey = code.clone();

                if code != "Control".to_string() && code != "Alt".to_string() && code != "Shift".to_string(){

                    if self.first{
                        data.one_key = false;
                    }
                    self.flag = true;
                    self.first = true;

                    if data.selected_shortcut == Shortcut::Screenshot{
                        data.keycode_screen = code2;
                    }else if data.selected_shortcut == Shortcut::Capture{
                        data.keycode_capture = code2;
                    }
                }

                if self.flag==true && (code == "Control".to_string() || code == "Alt".to_string() || code == "Shift".to_string()){
                    data.shortcut_order = false;
                }

                if code == "Control".to_string() || code == "Alt".to_string() || code == "Shift".to_string(){
                    data.with_modifiers = true;
                    if code == "Control".to_string(){
                        self.n_ctrl += 1;
                    }else if code == "Alt".to_string(){
                        self.n_alt += 1;
                    }else if code == "Shift".to_string(){
                        self.n_shift += 1;
                    }
                }

                if self.n_ctrl > 1 || self.n_alt > 1 || self.n_shift > 1{
                    data.dup_modifier = true;
                    println!("OP OP OP");
                }

                let shortcut: Vec<&str> = data.new_shortcut.split("+").collect();

                let save: Vec<&str> = data.shortcut.get(&Shortcut::Save).unwrap().split("+").collect();
                let save_as: Vec<&str> = data.shortcut.get(&Shortcut::SaveAs).unwrap().split("+").collect();
                let open: Vec<&str> = data.shortcut.get(&Shortcut::Open).unwrap().split("+").collect();
                let customize: Vec<&str> = data.shortcut.get(&Shortcut::Customize).unwrap().split("+").collect();
                let quit: Vec<&str> = data.shortcut.get(&Shortcut::Quit).unwrap().split("+").collect();
                let screenshot: Vec<&str> = data.shortcut.get(&Shortcut::Screenshot).unwrap().split("+").collect();
                let capture: Vec<&str> = data.shortcut.get(&Shortcut::Capture).unwrap().split("+").collect();

                if shortcut == save || shortcut == save_as || shortcut == open || shortcut == quit || shortcut == customize ||
                    shortcut == screenshot || shortcut == capture{
                    data.duplicate_shortcut = true;
                }

                if save.len() >= shortcut.len(){
                    if data.selected_shortcut!=Shortcut::Save && save.windows(shortcut.len()).any(|window| window == shortcut.as_slice()){
                        data.duplicate_shortcut = true;
                    }
                }else{
                    if data.selected_shortcut!=Shortcut::Save && shortcut.windows(save.len()).any(|window| window == save.as_slice()){
                        data.duplicate_shortcut = true;
                    }
                }

                if save_as.len() >= shortcut.len(){
                    if data.selected_shortcut!=Shortcut::SaveAs && save_as.windows(shortcut.len()).any(|window| window == shortcut.as_slice()){
                        data.duplicate_shortcut = true;
                    }
                }else{
                    if data.selected_shortcut!=Shortcut::SaveAs && shortcut.windows(save_as.len()).any(|window| window == save_as.as_slice()){
                        data.duplicate_shortcut = true;
                    }
                }

                if screenshot.len() >= shortcut.len(){
                    if data.selected_shortcut!=Shortcut::Screenshot && screenshot.windows(shortcut.len()).any(|window| window == shortcut.as_slice()){
                        data.duplicate_shortcut = true;
                    }
                }else{
                    if data.selected_shortcut!=Shortcut::Screenshot && shortcut.windows(screenshot.len()).any(|window| window == screenshot.as_slice()){
                        data.duplicate_shortcut = true;
                    }
                }

                if capture.len() >= shortcut.len(){
                    if data.selected_shortcut!=Shortcut::Capture && capture.windows(shortcut.len()).any(|window| window == shortcut.as_slice()){
                        data.duplicate_shortcut = true;
                    }
                }else{
                    if data.selected_shortcut!=Shortcut::Capture && shortcut.windows(capture.len()).any(|window| window == capture.as_slice()){
                        data.duplicate_shortcut = true;
                    }
                }

                if customize.len() >= shortcut.len(){
                    if data.selected_shortcut!=Shortcut::Customize && customize.windows(shortcut.len()).any(|window| window == shortcut.as_slice()){
                        data.duplicate_shortcut = true;
                    }
                }else{
                    if data.selected_shortcut!=Shortcut::Customize && shortcut.windows(customize.len()).any(|window| window == customize.as_slice()){
                        data.duplicate_shortcut = true;
                    }
                }

                if open.len() >= shortcut.len(){
                    if data.selected_shortcut!=Shortcut::Open && open.windows(shortcut.len()).any(|window| window == shortcut.as_slice()){
                        data.duplicate_shortcut = true;
                    }
                }else{
                    if data.selected_shortcut!=Shortcut::Open && shortcut.windows(open.len()).any(|window| window == open.as_slice()){
                        data.duplicate_shortcut = true;
                    }
                }

                if quit.len() >= shortcut.len(){
                    if data.selected_shortcut!=Shortcut::Quit && quit.windows(shortcut.len()).any(|window| window == shortcut.as_slice()){
                        data.duplicate_shortcut = true;
                    }
                }else{
                    if data.selected_shortcut!=Shortcut::Quit && shortcut.windows(quit.len()).any(|window| window == quit.as_slice()){
                        data.duplicate_shortcut = true;
                    }
                }
            }


        }
        child.event(ctx, event, data, _env);
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

pub struct HotkeyScreen{
    pub prec: String,
    pub code: String,
    pub timer: TimerToken,
    pub flag: bool,
}

impl<W: Widget<Screenshot>> Controller<Screenshot, W> for HotkeyScreen {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut Screenshot,
        _env: &Env,
    ) {

        if data.flag_focus{
            ctx.request_focus();
        }

        if self.flag == false{
            self.timer = ctx.request_timer(Duration::from_millis(100 as u64));
            self.flag= true;
        }

        if let Event::Timer(id) = event{

            if self.timer == *id{
            
                if data.receiver_app.is_full(){
                    let mx = data.receiver_app.recv();
                    match mx{
                        Ok(mx) => {
                            if mx == 1{
                                data.action_screen(ctx);
                            }else if mx==2{
                                data.action_capture(ctx);
                            }
                        }
                        Err(_) => panic!("panic shortcut!"),
                    }
                }
            }

            self.timer = ctx.request_timer(Duration::from_millis(100 as u64));
        }

        if let Event::KeyDown(key) = event{

            self.code = key.key.clone().to_string();
            if self.code != self.prec{
                if self.prec != "".to_string(){
                    data.new_shortcut.push_str("+");
                }
                data.new_shortcut.push_str(self.code.as_str());
                self.prec = self.code.clone();
            }
            let shortcut: Vec<&str> = data.new_shortcut.split("+").collect();

            let save: Vec<&str> = data.shortcut.get(&Shortcut::Save).unwrap().split("+").collect();
            let save_as: Vec<&str> = data.shortcut.get(&Shortcut::SaveAs).unwrap().split("+").collect();
            let open: Vec<&str> = data.shortcut.get(&Shortcut::Open).unwrap().split("+").collect();
            let customize: Vec<&str> = data.shortcut.get(&Shortcut::Customize).unwrap().split("+").collect();
            let quit: Vec<&str> = data.shortcut.get(&Shortcut::Quit).unwrap().split("+").collect();
         
            if shortcut == save{
                let image: ImageBuffer<image::Rgba<u8>, Vec<u8>> = ImageBuffer::from_vec(
                    data.img.width() as u32,
                    data.img.height() as u32,
                    data.img.raw_pixels().to_vec(),
                )
                .unwrap();

                data.new_shortcut.clear();
                self.prec.clear();

                image
                    .save_with_format(
                        format!(
                            "{}/{}{}",
                            data.default_save_path.clone(),
                            data.name,
                            data.format.to_string()
                        ),
                        image::ImageFormat::Png,
                    )
                    .expect("Errore nel salvataggio automatico!");
            }else if shortcut == save_as{
                let formats = vec![
                    FileSpec::new("jpg", &["jpg"]),
                    FileSpec::new("png", &["png"]),
                    FileSpec::new("gif", &["gif"]),
                    FileSpec::new("pnm", &["pnm"]),
                    FileSpec::new("tga", &["tga"]),
                    FileSpec::new("qoi", &["qoi"]),
                    FileSpec::new("tiff", &["tiff"]),
                    FileSpec::new("webp", &["webp"]),
                    FileSpec::new("bmp", &["bmp"]),
                ];

                let default_name = format!("{}{}", data.name, data.format.to_string());
                let save_dialog_options = FileDialogOptions::new()
                    .allowed_types(formats)
                    .default_type(FileSpec::new("png", &["png"]))
                    .default_name(default_name);

                data.new_shortcut.clear();
                self.prec.clear();
                ctx.submit_command(druid::commands::SHOW_SAVE_PANEL.with(save_dialog_options.clone()))
            }else if shortcut == open{
                let open_dialog_options: FileDialogOptions = FileDialogOptions::new()
                .select_directories()
                .button_text("Open");

                data.new_shortcut.clear();
                self.prec.clear();
                ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()))
            }else if shortcut == quit{
                Application::global().quit();
            }else if shortcut == customize{
                data.editing_shortcut = true;
                data.new_shortcut.clear();
                self.prec.clear();
                ctx.submit_command(SHORTCUT)
            }
        } 

        if let Event::KeyUp(_key) = event {
            data.new_shortcut.clear();
            self.prec.clear();
        }

        child.event(ctx, event, data, _env);
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

pub struct Drawer{
    pub flag_drawing: bool,
    pub flag_writing: bool,
    pub first_click_pos: Point,
}

impl<W: Widget<Screenshot>> Controller<Screenshot, W> for Drawer {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut Screenshot,
        _env: &Env,
    ) {
        if data.edit_tool == EditTool::Pencil{
            match event {
                Event::MouseDown(_mouse_event) => {
                    ctx.set_active(true);

                    let cursor_image = ImageBuf::from_data(include_bytes!("./svg/icons8-pencil-48.png")).unwrap();
                    data.custom_cursor_desc = CursorDesc::new(cursor_image, (0.0, 48.0));
                    data.custom_cursor = ctx.window().make_cursor(&data.custom_cursor_desc).unwrap_or(Cursor::Crosshair);
                    ctx.set_cursor(&data.custom_cursor);

                    let color = match data.color_tool{
                        ColorTool::Black => Color::BLACK,
                        ColorTool::Red => Color::RED,
                        ColorTool::Blue => Color::BLUE,
                        ColorTool::Yellow => Color::YELLOW,
                        ColorTool::White => Color::WHITE,
                        ColorTool::Green => Color::GREEN,
                    };
                    
                    
                    data.draw.points[data.draw.segment].3 = 1.;
                    
                    data.draw.points[data.draw.segment].1 = color;

                    data.draw.points[data.draw.segment].2 = data.line_thickness;

                    if data.line_thickness > 4.{
                        data.draw.points[data.draw.segment].2 = 4.;
                        data.line_thickness = 4.;
                    }
                    
                    self.flag_drawing = true;
                },
                Event::MouseMove(mouse_event) => {
                    if self.flag_drawing && is_in_image(mouse_event.pos, data) && ctx.is_active() {
                        data.draw.points[data.draw.segment].0.push_back(mouse_event.pos);
                    }
                },
                Event::MouseUp(_mouse_event) => {
                    ctx.set_active(false);
                    
                    data.draw.points.push_back((im::Vector::new(), Color::WHITE, 1., 1.));
                    
                    data.draw.segment += 1;
                    self.flag_drawing = false;
                    ctx.set_cursor(&Cursor::Arrow);

                },
                _ => ()
            }
        }
        else if data.edit_tool == EditTool::Highlighter{
            match event{
                Event::MouseDown(mouse_event) => {
                    ctx.set_active(true);
                    let color = match data.color_tool{
                        ColorTool::Black => Color::BLACK,
                        ColorTool::Red => Color::RED,
                        ColorTool::Blue => Color::BLUE,
                        ColorTool::Yellow => Color::YELLOW,
                        ColorTool::White => Color::WHITE,
                        ColorTool::Green => Color::GREEN,
                    };

                    let cursor_image = ImageBuf::from_data(include_bytes!("./svg/icons8-highlighter-48.png")).unwrap();
                    data.custom_cursor_desc = CursorDesc::new(cursor_image, (0., 48.0));
                    data.custom_cursor = ctx.window().make_cursor(&data.custom_cursor_desc).unwrap_or(Cursor::Crosshair);
                    ctx.set_cursor(&data.custom_cursor);

                    data.draw_high.0[data.draw_high.1].start = mouse_event.pos;
                    data.draw_high.0[data.draw_high.1].end = mouse_event.pos;
                    data.draw_high.0[data.draw_high.1].color = color;
                    data.draw_high.0[data.draw_high.1].thickness = data.line_thickness;
                    data.draw_high.0[data.draw_high.1].alpha = 0.5;
                    
                }
                Event::MouseMove(mouse_event) => {
                    if ctx.is_active() && is_in_image(mouse_event.pos, data) {
                        data.draw_high.0[data.draw_high.1].end = mouse_event.pos;
                    }
                }
                Event::MouseUp(_mouse_event) => {
                    ctx.set_active(false);
                    data.draw_high.0.push_back(Highlighter::new());
                    data.draw_high.1 += 1;
                }
                _ => {}
            }
        }
        else if data.edit_tool == EditTool::Text{
            match event{
                Event::MouseDown(mouse_event) => {
                    self.flag_writing = true;

                    data.write.0[data.write.1].position = mouse_event.pos;

                    let ev_x = mouse_event.pos.x;
                    let ev_y = mouse_event.pos.y;

                    for (index, text) in data.write.0.iter().enumerate(){
                        let txt_x = text.position.x;
                        let txt_y = text.position.y;
                        let w = text.dimensions.0;
                        let h = text.dimensions.1;
                        if ev_x > txt_x && ev_x < txt_x + w && ev_y > txt_y && ev_y < txt_y + h {
                            data.editing_text = index as i32;
                            self.first_click_pos = mouse_event.pos;
                            ctx.set_active(true);
                            ctx.set_cursor(&Cursor::Pointer);
                            break;
                        }else{
                            data.text = "".to_string();
                            data.editing_text = -1;
                        }
                    }

                    if data.editing_text != -1 {
                        data.text = data.write.0[data.editing_text as usize].text.clone();
                    }

                }
                Event::MouseMove(mouse_event) => {
                    if ctx.is_active() {

                        let txt_w =  data.write.0[data.editing_text as usize].dimensions.0;
                        let txt_h =  data.write.0[data.editing_text as usize].dimensions.1;
                        let pos_init = data.write.0[data.editing_text as usize].position;
                        let pos_final = Point::new(data.write.0[data.editing_text as usize].position.x + txt_w, data.write.0[data.editing_text as usize].position.y + txt_h);



                        if data.editing_text != -1 && is_in_image(mouse_event.pos, data){

                            let delta_x = mouse_event.pos.x - self.first_click_pos.x;
                            let delta_y = mouse_event.pos.y - self.first_click_pos.y;
                            self.first_click_pos = mouse_event.pos;
                            if is_in_image(Point::new(pos_init.x + delta_x, pos_init.y + delta_y), data) && is_in_image(Point::new(pos_final.x + delta_x, pos_final.y + delta_y), data){
                                data.write.0[data.editing_text as usize].position = Point::new(pos_init.x + delta_x, pos_init.y + delta_y);
                            }
                        }
                    }

                }
                Event::MouseUp(_ev) => {
                    ctx.set_cursor(&Cursor::Arrow);
                    ctx.set_active(false);
                }
                _ => {}
            }
        }
        else if data.edit_tool == EditTool::Shape{
            if data.shape_tool == ShapeTool::Arrow{
                match event{
                    Event::MouseDown(mouse_event) => {
                        ctx.set_active(true);
                        let color = match data.color_tool{
                            ColorTool::Black => Color::BLACK,
                            ColorTool::Red => Color::RED,
                            ColorTool::Blue => Color::BLUE,
                            ColorTool::Yellow => Color::YELLOW,
                            ColorTool::White => Color::WHITE,
                            ColorTool::Green => Color::GREEN,
                        };
                        data.arrows.0[data.arrows.1].start = mouse_event.pos;
                        data.arrows.0[data.arrows.1].end = mouse_event.pos;
                        data.arrows.0[data.arrows.1].color = color;
                        data.arrows.0[data.arrows.1].thickness = data.line_thickness;
                    }
                    Event::MouseMove(mouse_event) => {
                        if ctx.is_active() && is_in_image(mouse_event.pos, data) {
                            data.arrows.0[data.arrows.1].end = mouse_event.pos;
                        }
                    }
                    Event::MouseUp(_mouse_event) => {
                        ctx.set_active(false);
                        data.arrows.0.push_back(Arrow::new());
                        data.arrows.1 += 1;
                    }
                    _ => {}
                }
            }else if data.shape_tool == ShapeTool::Circle{
                match event{
                    Event::MouseDown(mouse_event) => {
                        ctx.set_active(true);
                        let color = match data.color_tool{
                            ColorTool::Black => Color::BLACK,
                            ColorTool::Red => Color::RED,
                            ColorTool::Blue => Color::BLUE,
                            ColorTool::Yellow => Color::YELLOW,
                            ColorTool::White => Color::WHITE,
                            ColorTool::Green => Color::GREEN,
                        };
                        data.circles.0[data.circles.1].start = mouse_event.pos;
                        data.circles.0[data.circles.1].end = mouse_event.pos;
                        data.circles.0[data.circles.1].color = color;
                        data.circles.0[data.circles.1].thickness = data.line_thickness;
                    }
                    Event::MouseMove(mouse_event) => {
                        if ctx.is_active() && is_in_image(mouse_event.pos, data) {
                            data.circles.0[data.circles.1].end = mouse_event.pos;
                        }
                    }
                    Event::MouseUp(_mouse_event) => {
                        ctx.set_active(false);
                        data.circles.0.push_back(Circle::new());
                        data.circles.1 += 1;
                    }
                    _ => {}
                }
            }else if data.shape_tool == ShapeTool::Square{
                match event{
                    Event::MouseDown(mouse_event) => {
                        ctx.set_active(true);
                        let color = match data.color_tool{
                            ColorTool::Black => Color::BLACK,
                            ColorTool::Red => Color::RED,
                            ColorTool::Blue => Color::BLUE,
                            ColorTool::Yellow => Color::YELLOW,
                            ColorTool::White => Color::WHITE,
                            ColorTool::Green => Color::GREEN,
                        };
                        data.squares.0[data.squares.1].start = mouse_event.pos;
                        data.squares.0[data.squares.1].end = mouse_event.pos;
                        data.squares.0[data.squares.1].color = color;
                        data.squares.0[data.squares.1].thickness = data.line_thickness;
                    }
                    Event::MouseMove(mouse_event) => {
                        if ctx.is_active() && is_in_image(mouse_event.pos, data) {
                            data.squares.0[data.squares.1].end = mouse_event.pos;
                        }
                    }
                    Event::MouseUp(_mouse_event) => {
                        ctx.set_active(false);
                        data.squares.0.push_back(Square::new());
                        data.squares.1 += 1;
                    }
                    _ => {}
                }
            }
        }
        else if data.edit_tool == EditTool::Eraser{
            match event{
                Event::MouseDown(_mouse_event) => {
                    ctx.set_active(true);

                    let cursor_image = ImageBuf::from_data(include_bytes!("./svg/eraser2-30.png")).unwrap();
                    data.custom_cursor_desc = CursorDesc::new(cursor_image, (0.0, 30.0));

                    data.custom_cursor = ctx.window().make_cursor(&data.custom_cursor_desc).unwrap_or(Cursor::Crosshair);
                    ctx.set_cursor(&data.custom_cursor);
                },
                Event::MouseMove(mouse_event) => {
                    if ctx.is_active(){

                        //erase free hand shapes
                        for (index, track) in data.draw.points.clone().iter().enumerate(){ //check all tracks present on the image
                            for point in track.0.clone() { //check all the points of each track
                                if mouse_event.pos.distance(point) < 10. { //if an intersection is found, remove the entire track from the draw
                                    data.draw.points.remove(index);
                                    data.draw.segment -= 1;
                                    break;
                                }
                            }
                        }

                         //erase highlighters
                         for (index, high) in data.draw_high.0.clone().iter().enumerate(){
                            for p in arrow_body_points(high.start, high.end) {
                                if mouse_event.pos.distance(p) < 10. {
                                    data.draw_high.0.remove(index);
                                    data.draw_high.1 -= 1;
                                    break;
                                }
                            }
                        }

                        //erase arrows
                        for (index, arrow) in data.arrows.0.clone().iter().enumerate(){
                            for p in arrow_body_points(arrow.start, arrow.end) {
                                if mouse_event.pos.distance(p) < 10. {
                                    data.arrows.0.remove(index);
                                    data.arrows.1 -= 1;
                                    break;
                                }
                            }
                        }

                        //erase squares
                        for(index, square) in data.squares.0.clone().iter().enumerate(){
                            if is_in_square(mouse_event.pos, square.start, square.end){
                                data.squares.0.remove(index);
                                data.squares.1 -= 1;
                                break;
                            }
                        }

                        //erase circles
                        for(index, circle_sq) in data.circles.0.clone().iter().enumerate(){
                            if is_in_square(mouse_event.pos, circle_sq.start, circle_sq.end){
                                data.circles.0.remove(index);
                                data.circles.1 -= 1;
                                break;
                            }
                        }
                    }
                },
                Event::MouseUp(_mouse_event) => {
                    ctx.set_active(false);
                    ctx.set_cursor(&Cursor::Arrow);
                },
                _ => ()
            }
        }

        child.event(ctx, event, data, _env);
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

fn is_in_image(point: Point, data: &Screenshot) -> bool{
    point.x >= data.resized_area.x &&
    point.y >= data.resized_area.y &&
    point.x <= data.resized_area.x + data.resized_area.width &&
    point.y <= data.resized_area.y + data.resized_area.height
}

fn is_in_square(point: Point, square_start: Point, square_end: Point) -> bool{
    let p0 = Point::new(min(square_start.x, square_end.x), min(square_start.y, square_end.y));
    let p1 = Point::new(max(square_start.x, square_end.x), max(square_start.y, square_end.y));
    point.x >= p0.x &&
    point.y >= p0.y &&
    point.x <= p1.x &&
    point.y <= p1.y
}

fn arrow_body_points(start: Point, end: Point) -> Vec<Point>{
    let mut segment = Vec::new();

    // segment lenght
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let len = (dx * dx + dy * dy).sqrt();

    // segment direction
    let dir_x = dx / len;
    let dir_y = dy / len;

    // Calcoliamo i punti intermedi sul segmento
    for t in 0..=len as usize {
        let x = start.x + t as f64 * dir_x;
        let y = start.y + t as f64 * dir_y;
        segment.push(Point::new(x, y));
    }
    segment
}

fn min(val1: f64, val2:f64) -> f64 {
    if val1<val2{
        return val1
    };
    val2
}

fn max(val1: f64, val2:f64) -> f64 {
    if val1>val2{
        return val1
    };
    val2
}