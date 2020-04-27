use std::ops;

pub struct Circle {
    pub freq: i32,
    pub amp: f64,
    pub phase: f64
}

#[derive(Copy, Clone)]
pub struct Complex {
    pub img: f64,
    pub real: f64,
}

impl ops::Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        Complex { real: self.real + rhs.real, img: self.img + rhs.img }
    }
}

impl ops::Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real * rhs.real - self.img * rhs.img,
            img: self.real * rhs.img + self.img * rhs.real,
        }
    }
}

pub struct Fourier { }

impl Fourier {

    pub fn fourier(data_points: &Vec<Complex>) -> Vec<Circle> {
        use std::f64::consts::PI;

        let mut circles = vec![];
        let range = data_points.len();
        let data_len = data_points.len();

        for k in 0..range {
            let mut sum = Complex { real: 0., img: 0. };
            for n in 0..data_len {
                let point = data_points[n];
                let shift = (PI * 2.) * (k as f64) * (n as f64) / (range as f64);
                let rotation = Complex { real: shift.cos(), img: -shift.sin() };
                let rotated = rotation * point;
                sum = sum + rotated;
            }

            sum.real = sum.real / data_len as f64;
            sum.img = sum.img / data_len as f64;
            let line_len = (sum.real * sum.real + sum.img * sum.img).sqrt();
            let angle = sum.img.atan2(sum.real);

            let circle = Circle { freq: k as i32, amp: line_len, phase: angle };
            circles.push(circle);

        }
        circles.sort_by(|a, b| a.amp.partial_cmp(&b.amp).unwrap() );
        circles.reverse();
        circles
    }

}
