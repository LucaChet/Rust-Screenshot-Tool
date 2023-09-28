use druid::{
    Selector, WindowDesc, KeyEvent, KbKey, commands, AppDelegate, Code, Command, Cursor, DelegateCtx, Env, Event, EventCtx,
    FileDialogOptions, FileSpec, Handled, LocalizedString, MouseButton, Point, Target, Widget,
    WindowState,
};
use std::time::Duration;

use druid::widget::Controller;
use druid_shell::TimerToken;

use crate::ui::*;
use crate::data::*;
use image::*;
pub const SHORTCUT: Selector = Selector::new("shortcut_selector");
pub struct SetScreen;

impl<W: Widget<Screenshot>> Controller<Screenshot, W> for SetScreen {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut Screenshot,
        env: &Env,
    ) {
        data.area.start = Point::new(0.0, 0.0);
        data.area.end = Point::new(0.0, 0.0);
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

            // if data.monitor_id != 0{
            //     self.flag_desk2 = true;
            // }

            if data.time_interval > 0.0 && self.flag {
                self.t1 = ctx.request_timer(Duration::from_secs(data.time_interval as u64));
                self.flag = false;
                current.set_window_state(WindowState::Minimized);
            } else if self.flag {
                self.t1 = ctx.request_timer(Duration::from_millis(100));
                self.flag = false;
                current.set_window_state(WindowState::Minimized);
                ctx.set_cursor(&Cursor::Crosshair);
            }
            match event {
                Event::MouseDown(mouse_event) => {
                    if mouse_event.button == MouseButton::Left {
                        let start_point = mouse_event.pos;

                        ctx.set_active(true);
                        // ctx.set_handled();

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
                        self.t1 = ctx.request_timer(Duration::from_millis(500));

                        ctx.set_active(false);
                        // ctx.set_handled();

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
                        if data.monitor_id != 0{
                            println!("km");
                            data.do_screen();
                            data.flag_desk2 = true;
                            // self.t1 = ctx.request_timer(Duration::from_millis(100));
                        }
                        if data.flag_desk2 == true {
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
        child: &mut W,
        ctx: &mut EventCtx,
        event: &druid::Event,
        data: &mut Screenshot,
        env: &Env,
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
            Event::MouseUp(mouse_event) => {
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
            // let img_bytes: &[u8] = data.img.raw_pixels();
            // if let Err(e) = std::fs::write(file_info.path(), img_bytes) {
            //     println!("Error writing file: {e}");
            // }
            // Specifica il formato dell'immagine (in questo caso PNG)
            // if let Some(path) = file_info.pa
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
                    println!("{}", String::from(file_info.path().to_str().unwrap()));
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

pub struct HotKeyController;

impl<W: Widget<Screenshot>> Controller<Screenshot, W> for HotKeyController {
    fn event( 
        &mut self, 
        child: &mut W, 
        ctx: &mut EventCtx, 
        event: &Event, 
        data: &mut Screenshot, 
        _env: &Env, 
    ) { 
        if let Event::KeyDown(key) = event {

            data.duplicate_shortcut = false;
            if key.code == Code::Enter{
                data.editing_shortcut = false;
                ctx.window().close();
            }else{
                if (key.code.to_string() >= "Digit0".to_string() && key.code.to_string() <= "Digit9".to_string())
                || (key.code.to_string() >= "KeyA".to_string() && key.code.to_string() <= "KeyZ".to_string()){
                    
                    let code = key.code.to_string().chars().last().unwrap().to_string().to_lowercase();
                    data.new_name = "".to_string();

                    for val in data.shortcut.values(){
                        if code == *val{
                            data.duplicate_shortcut = true;
                        }
                    }

                    if !data.duplicate_shortcut{
                        match data.selected_shortcut{
                            Shortcut::Save => {
                                data.shortcut.entry(Shortcut::Save).and_modify(|el| *el = code);
                            }
                            Shortcut::Open => {
                                data.shortcut.entry(Shortcut::Open).and_modify(|el| *el = code);
                            }
                            Shortcut::SaveAs => {
                                data.shortcut.entry(Shortcut::SaveAs).and_modify(|el| *el = code);
                            }
                            Shortcut::Quit => {
                                data.shortcut.entry(Shortcut::Quit).and_modify(|el| *el = code);
                            }
                            Shortcut::Customize => {
                                data.shortcut.entry(Shortcut::Customize).and_modify(|el| *el = code);
                            }
                            Shortcut::Screenshot => {
                                data.shortcut.entry(Shortcut::Screenshot).and_modify(|el| *el = code);
                            }
                            Shortcut::Capture => {
                                data.shortcut.entry(Shortcut::Capture).and_modify(|el| *el = code);
                            }
                        }
                    }
                }
            }
        }
        child.event(ctx, event, data, _env); 
    } 
}

pub struct HotkeyScreen{
    pub flag_ctrl: bool,
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
        let mut code = "".to_string();
        ctx.request_focus();

        if let Event::KeyDown(key) = event {
            if key.code == Code::ControlLeft{
                self.flag_ctrl = true;
            }
        }

        if let Event::KeyUp(key) = event {
            if key.code == Code::ControlLeft{
                self.flag_ctrl = false;
            }
        }

        if let Event::KeyDown(key) = event{
            let screen = data.shortcut.get(&Shortcut::Screenshot).unwrap().as_str();
            let capture = data.shortcut.get(&Shortcut::Capture).unwrap().as_str();

            if (key.code.to_string() >= "Digit0".to_string() && key.code.to_string() <= "Digit9".to_string())
            || (key.code.to_string() >= "KeyA".to_string() && key.code.to_string() <= "KeyZ".to_string()){
                code = key.code.to_string().chars().last().unwrap().to_string().to_lowercase();
            }

            if code == screen && self.flag_ctrl{
                data.action_screen(ctx);
            }else if code == capture && self.flag_ctrl{
                data.action_capture(ctx);
            }
        }

        child.event(ctx, event, data, _env);
    }
}