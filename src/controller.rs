use druid::{
    commands, AppDelegate, Code, Command, Cursor, DelegateCtx, Env, Event, EventCtx,
    FileDialogOptions, FileSpec, Handled, LocalizedString, MouseButton, Point, Target, Widget,
    WindowState,
};
use std::time::Duration;

use druid::widget::Controller;
use druid_shell::TimerToken;

use crate::data::*;
use image::*;

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

            if data.time_interval > 0.0 && self.flag {
                self.t1 = ctx.request_timer(Duration::from_secs(data.time_interval as u64));
                self.flag = false;
                current.set_window_state(WindowState::Minimized);
            } else if self.flag {
                self.flag = false;
                ctx.set_cursor(&Cursor::Crosshair);
            }
            match event {
                Event::MouseDown(mouse_event) => {
                    if mouse_event.button == MouseButton::Left {
                        // Esegui qualcosa quando viene premuto il pulsante sinistro del mouse.
                        // Ad esempio, puoi iniziare a trascinare un elemento.
                        // Inizia a tenere traccia del punto in cui è iniziato il trascinamento.

                        // ctx.set_cursor(&Cursor::Crosshair);

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

                        data.flag_transparency = true;

                        data.area.rgba.r = 0.0;
                        data.area.rgba.g = 0.0;
                        data.area.rgba.b = 0.0;
                        data.area.rgba.a = 0.0;
                        data.flag_selection = true;
                        self.t1 = ctx.request_timer(Duration::from_millis(500));

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
                        data.time_interval = 0.0;
                        current.set_always_on_top(true);
                        current.set_window_state(WindowState::Restored);
                        ctx.set_cursor(&Cursor::Crosshair);
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

pub struct ResizeController;

impl<W: Widget<Screenshot>> Controller<Screenshot, W> for ResizeController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &druid::Event,
        data: &mut Screenshot,
        env: &Env,
    ) {
        let area_width = 800.;
        let area_height = 500.;
        let original_width = data.img.width() as f64;
        let original_height = data.img.height() as f64;

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

        data.resized_area.x = top_left_x;
        data.resized_area.y = top_left_y;
        data.resized_area.width = new_width;
        data.resized_area.height = new_height;

        let delta = 1.0;

        match event {
            Event::MouseDown(mouse_event) => {}
            Event::MouseUp(mouse_event) => {}
            Event::MouseMove(mouse_event) => {

                // Controlla il bordo superiore.
                if mouse_event.pos.x >= data.resized_area.x
                    && mouse_event.pos.x <= data.resized_area.x + data.resized_area.width
                    && mouse_event.pos.y >= data.resized_area.y
                    && mouse_event.pos.y < data.resized_area.y + 3.0
                {
                    ctx.set_cursor(&Cursor::ResizeUpDown);
                }
                // Controlla il bordo inferiore.
                else if mouse_event.pos.x >= data.resized_area.x
                    && mouse_event.pos.x <= data.resized_area.x + data.resized_area.width
                    && mouse_event.pos.y >= data.resized_area.y + data.resized_area.height - 3.0
                    && mouse_event.pos.y <= data.resized_area.y + data.resized_area.height
                {
                    ctx.set_cursor(&Cursor::ResizeUpDown);
                }
                // Controlla il bordo destro.
                else if mouse_event.pos.x >= data.resized_area.x + data.resized_area.width - 3.0
                    && mouse_event.pos.x <= data.resized_area.x + data.resized_area.width
                    && mouse_event.pos.y >= data.resized_area.y
                    && mouse_event.pos.y <= data.resized_area.y + data.resized_area.height
                {
                    ctx.set_cursor(&Cursor::ResizeLeftRight);
                }
                // Controlla il bordo sinistro.
                else if mouse_event.pos.x >= data.resized_area.x
                    && mouse_event.pos.x < data.resized_area.x + 3.0
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
                }else{
                    ctx.set_cursor(&Cursor::Arrow);
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
        Handled::No
    }
}
