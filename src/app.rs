use piston::input::{RenderArgs, UpdateArgs};
use opengl_graphics::{GlGraphics};

mod line;

pub struct App {
    gl: GlGraphics,
    lines: Vec<line::Line>,
    points: Vec<[f64; 2]>,
    data: Vec<Vec<[u8; 3]>>,
}

impl App {
    pub fn new(gl: glutin_window::OpenGL, width: u32, height: u32, data: Vec<Vec<[u8; 3]>>) -> App {
        let mut line = 
            line::Line::new(
                [(width / 2) as f64, (height / 2) as f64], 75., 0., 1., 0
                );

        line.grow(10);
        let end = line.last().end();
        App {
            gl: GlGraphics::new(gl),
            lines: vec![ line ],
            points: vec![ end ],
            data: data
        }
    }

    pub fn draw_data(&mut self, args: &RenderArgs) {
        use graphics::*;
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [255.0, 255.0, 255.0, 1.0];

        let data = &self.data;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);
            let rect = [0., 0., 1., 1.];

            for x in 0..data.len() {
                for y in 0..data[x].len() {
                    let transform = c.transform.trans(x as f64, y as f64);
                    let point = data[x][y];
                    let color: [f32; 4] = [point[0] as f32, point[1] as f32, point[2] as f32, 1.];
                    rectangle(color, rect, transform, gl);
                }
            }
        });

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
