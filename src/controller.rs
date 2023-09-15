use druid::{
    commands, AppDelegate, Code, Command, Cursor, DelegateCtx, Env, Event, EventCtx, Handled,
    MouseButton, Point, Target, Widget,
};

use druid::widget::Controller;

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
        if data.flag_selection && data.area_transparency == 0.0 && !ctx.is_active() {
            if data.area.width != 0.0 && data.area.heigth != 0.0 {
                data.do_screen_area();
                // data.area_transparency = 0.4;
            }
            data.area.start = Point::new(0.0, 0.0);
            data.area.end = Point::new(0.0, 0.0);
            data.flag_selection = false;
            ctx.window().close();
        } else if data.window_minimized {
            data.do_screen();
            data.area.start = Point::new(0.0, 0.0);
            data.area.end = Point::new(0.0, 0.0);
            data.window_minimized = false;
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
        ctx: &mut EventCtx,
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
        Handled::No
    }
}