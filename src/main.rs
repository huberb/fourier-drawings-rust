extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

mod app;
mod reader;

fn main() {
    let opengl = OpenGL::V3_2;

    let reader = reader::Reader::new("/Users/ben/Desktop/drawing/misc/musical-note.png".to_string());
    let (width, height, data) = reader.read_as_line_img();

    let mut window: Window = WindowSettings::new("circles", [width, height])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = app::App::new(opengl, width, height, data);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.draw_data(&args);
        }
        // if let Some(args) = e.render_args() {
        //     app.render(&args);
        // }

        // if let Some(args) = e.update_args() {
        //     app.update(&args);
        // }
    }
}
