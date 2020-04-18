pub struct Line {
    start: [f64; 2],
    length: f64,
    rotation: f64,
    speed: f64,
    index: i64,
    pub next_line: Box<Option<Line>>,
}

impl Line {
    pub fn new(start: [f64; 2], length: f64, rotation: f64, speed: f64, index: i64) -> Line {
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

    pub fn grow(&mut self, to: i64) {
        self.push();
        let mut line = &mut *self.next_line;
        while let Some(next) = line {
            if next.index == to { return }
            next.push();
            line = &mut *next.next_line;
        }
    }

    pub fn update(&mut self, dt: f64) -> [f64; 2] {
        self.rotation += self.speed * dt;
        let end = self.end();
        if let Some(next_line) = &mut *self.next_line {
            next_line.start = end;
            return next_line.update(dt);
        } else {
            return end;
        }
    }

    pub fn last(&self) -> &Line {
        let mut line = self;
        while let Some(next) = &*line.next_line {
            line = next;
        }
        line
    }

    pub fn start(&self) -> [f64; 2] {
        self.start
    }

    pub fn end(&self) -> [f64; 2] {
        let x = self.rotation.cos();
        let y = self.rotation.sin();
        [
            x * self.length + self.start[0],
            y * self.length + self.start[1]
        ]
    }
}
