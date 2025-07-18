use std::time::{Instant};

pub struct Fps {
    frame_times: [f64; 100],
    index: usize,
    timer: Instant,
    full: bool,
}

impl Fps {
    pub fn start() -> Self {
        Self {
            frame_times: [0.0; 100],
            index: 0,
            timer: Instant::now(),
            full: false,
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.timer).as_secs_f64();
        self.timer = now;

        self.frame_times[self.index] = delta;
        self.index = (self.index + 1) % self.frame_times.len();
        if self.index == 0 {
            self.full = true;
        }
    }

    pub fn read(&self) -> f64 {
        let len = if self.full { self.frame_times.len() } else { self.index };
        let total_time: f64 = self.frame_times[..len].iter().sum();
        if total_time == 0.0 {
            0.0
        } else {
            len as f64 / total_time
        }
    }
}
