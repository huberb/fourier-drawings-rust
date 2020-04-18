use piston::input::{RenderArgs, UpdateArgs};
use opengl_graphics::{GlGraphics};

mod line;

pub struct App {
    gl: GlGraphics,
    lines: Vec<line::Line>,
    points: Vec<[f64; 2]>,
}

impl App {
    pub fn new(gl: glutin_window::OpenGL, width: f64, height: f64) -> App {
        let mut line = 
            line::Line::new(
                [width / 2., height / 2.], 75., 0., 1., 0
                );

        line.grow(10);
        let end = line.last().end();
        App {
            gl: GlGraphics::new(gl),
            lines: vec![ line ],
            points: vec![ end ],
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

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

    pub fn update(&mut self, args: &UpdateArgs) {
        for line in &mut self.lines {
            let point = line.update(args.dt);
            self.points.push(point);
        }
    }
}
