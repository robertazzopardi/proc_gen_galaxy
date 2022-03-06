use std::time::{Duration, Instant};

use procedural_gen::State;

const FRAME_TIME: f32 = 0.01666;

pub struct Time {
    dt: Duration,
    t: Duration,
    now: Instant,
    acc: Duration,
}

impl Time {
    pub fn new() -> Self {
        Self {
            dt: Duration::from_secs_f32(FRAME_TIME),
            t: Duration::ZERO,
            now: std::time::Instant::now(),
            acc: Duration::ZERO,
        }
    }

    pub fn diff(&mut self) {
        let new_time = std::time::Instant::now();
        let frame_time = new_time - self.now; // from ns to s
        self.now = new_time;
        self.acc += frame_time;
    }

    pub(crate) fn update(&mut self, state: &mut State) {
        while self.acc >= self.dt {
            state.update(self.dt.as_secs_f32());
            self.acc -= self.dt;
            self.t += self.dt;
        }
    }
}
