use std::time::Instant;


pub struct Fps {
    timer: Instant,
    count: usize,
    fps: u16
}

impl Fps {

    pub fn start() -> Self {
        Self { timer: Instant::now(), count: 0, fps: 0 }
    }

    pub fn tick(&mut self) {
        self.count += 1;
        if self.timer.elapsed().as_secs_f64() > 1.0 {
            self.cal();
        }
    }

    fn cal(&mut self) {
        self.fps = (self.count as f64 / self.timer.elapsed().as_secs_f64()) as u16;
        self.timer = Instant::now();
        self.count = 0;
    }

    pub fn read(&self) -> u16 {
        self.fps
    }

}