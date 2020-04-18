extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

struct Line {
    start: [f64; 2],
    length: f64,
    rotation: f64,
    speed: f64,
    next_line: Box<Option<Line>>,
    index: i64,
}

impl Line {
    fn new(start: [f64; 2], length: f64, rotation: f64, speed: f64, index: i64) -> Line {
        Line { start, length, rotation, next_line: Box::new(None), speed, index }
    }

    fn push(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        if let None = *self.next_line {
            self.next_line = 
                Box::new(
                    Some(
                        Line::new(
                            self.end(), 
                            self.length * 0.9, 
                            self.rotation * -1.,
                            rng.gen::<f64>() * 5.,
                            self.index + 1
                            )
                        )
                    );
        }
    }

    fn grow(&mut self, to: i64) {
        self.push();
        let mut line = &mut *self.next_line;
        while let Some(next) = line {
            if next.index == to { return }
            next.push();
            line = &mut *next.next_line;
        }
    }

    fn update(&mut self, dt: f64) -> [f64; 2] {
        self.rotation += self.speed * dt;
        let end = self.end();
        if let Some(next_line) = &mut *self.next_line {
            next_line.start = end;
            return next_line.update(dt);
        } else {
            return end;
        }
    }

    fn last(&self) -> &Line {
        let mut line = self;
        while let Some(next) = &*line.next_line {
            line = next;
        }
        line
    }

    fn start(&self) -> [f64; 2] {
        self.start
    }

    fn end(&self) -> [f64; 2] {
        let x = self.rotation.cos();
        let y = self.rotation.sin();
        [
            x * self.length + self.start[0],
            y * self.length + self.start[1]
        ]
    }
}

pub struct App {
    gl: GlGraphics,
    lines: Vec<Line>,
    points: Vec<[f64; 2]>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        // let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        // let width = args.window_size[0];
        // let height = args.window_size[1];

        let lines = &self.lines;
        let points = &self.points;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            let mut point_from = points[0];
            for i in 1..points.len() {
                let point_to = points[i];
                line_from_to(GREEN, 0.5, point_from, point_to, c.transform, gl);
                point_from = point_to;
            }

            for mut line in lines {
                line_from_to(RED, 0.5, line.start(), line.end(), c.transform, gl);
                while let Some(next) = &*line.next_line {
                    line_from_to(RED, 0.5, next.start(), next.end(), c.transform, gl);
                    line = &*next;
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        for line in &mut self.lines {
            let point = line.update(args.dt);
            self.points.push(point);
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let width = 512.;
    let height = 512.;

    let mut window: Window = WindowSettings::new("circles", [width, height])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut line = Line::new([width / 2., height / 2.], 75., 0., 1., 0);
    line.grow(10);
    let end = line.last().end();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        lines: vec![ line ],
        points: vec![ end ],
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
