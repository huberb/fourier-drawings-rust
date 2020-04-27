use piston::input::{RenderArgs, UpdateArgs};
use opengl_graphics::{GlGraphics};

pub mod fourier;
use fourier::{Circle, Complex, Fourier};

pub struct App {
    gl: GlGraphics,
    width: u32,
    height: u32,
    time: f64,
    length: usize,
    circles: Vec<Circle>,
    points: Vec<Complex>,
}

impl App {
    pub fn new(gl: glutin_window::OpenGL, width: u32, height: u32) -> App {
        App { gl: GlGraphics::new(gl), width, height, points: vec![], circles: vec![], time: 0., length: 0 }
    }

    pub fn start(&mut self, points: &Vec<[u32; 2]>) {

        let center_x = (self.width / 2) as f64;
        let center_y = (self.width / 2) as f64;

        let numbers = points.iter().map(|p| {
            fourier::Complex { 
                real: p[0] as f64 - center_x, 
                img: p[1] as f64 - center_y
            }
        }).collect();

        let circles = Fourier::fourier(&numbers);

        self.points = vec![];
        self.circles = circles;
        self.time = 0.;
        self.length = self.circles.len();
    }

    pub fn draw_points(&mut self, points: &Vec<[u32; 2]>, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            let mut last_x = points[0][0] as f64;
            let mut last_y = points[0][1] as f64;
            for i in 1..points.len() {
                let point = points[i];
                let x = point[0] as f64;
                let y = point[1] as f64;
                line_from_to(WHITE, 1., [last_x, last_y], [x, y], c.transform, gl);
                last_x = x;
                last_y = y;
            }
        })
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let circles = &self.circles;
        let points = &self.points;
        let time = &self.time;

        let center_x = (self.width / 2) as f64;
        let center_y = (self.height / 2) as f64;

        let mut last_x = center_x;
        let mut last_y = center_y;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            if points.len() > 1 {
                let mut last_x = points[0].real as f64;
                let mut last_y = points[0].img as f64;
                for i in 1..points.len() {
                    let point = points[i];
                    let x = point.real as f64;
                    let y = point.img as f64;
                    line_from_to(WHITE, 1., [last_x, last_y], [x, y], c.transform, gl);
                    last_x = x;
                    last_y = y;
                }
            }

            for circle in circles {
                let x = last_x;
                let y = last_y;
                last_x += circle.amp * (circle.freq as f64 * time + circle.phase).cos();
                last_y += circle.amp * (circle.freq as f64 * time + circle.phase).sin();
                line_from_to(GREEN, 1., [x, y], [last_x, last_y], c.transform, gl);
                let rect = [0., 0., circle.amp * 2., circle.amp * 2.];
                let offset = c.transform.trans(x - circle.amp, y - circle.amp);
                circle_arc(RED, 0.3, 0., 3.14 * 2., rect, offset, gl);
            }

        });
        self.points.push(fourier::Complex { real: last_x, img: last_y });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        let two_pi = std::f64::consts::PI * 2.;
        if self.length > 0 {
            let dt = two_pi / self.length as f64;
            self.time = self.time + dt;
            if self.time > two_pi {
                self.points = vec![];
                self.time = 0.;
            }
        }
    }
}
