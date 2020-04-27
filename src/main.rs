extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::input::*;

mod app;

fn main() {
    let opengl = OpenGL::V3_2;
    let width = 750;
    let height = 750;

    let mut drawing = false;
    let mut points = vec![];

    let mut window: Window = WindowSettings::new("circles", [width, height])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = app::App::new(opengl, width, height);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        if let Some(args) = e.render_args() {
            if drawing && points.len() > 1 {
                app.draw_points(&points, &args);
            } else {
                app.render(&args);
            }
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(Button::Mouse(_button)) = e.press_args() {
            drawing = true;
        }

        if let Some(Button::Mouse(_button)) = e.release_args() {
            drawing = false;
            app.start(&points);
            points = vec![];
        }

        if drawing {
            e.mouse_cursor(|pos| { points.push([pos[0] as u32, pos[1] as u32]); });
        }
    }
}
