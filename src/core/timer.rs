use std::time::{Instant};

pub struct Timer {
    last_instant: f32,
    internal_timer: Instant,
}

impl Timer {
    pub fn new() -> Timer {
        Timer { 
            last_instant: 0.0,
            internal_timer: Instant::now()
        }
    }

    pub fn get_delta(&mut self) -> f32 {
        let cur_secs = self.internal_timer.elapsed().as_secs() as f32;
        let cur_nanos_as_secs = (self.internal_timer.elapsed().subsec_nanos() as f32) / 1_000_000_000.0;
        let next_instant = (cur_secs + cur_nanos_as_secs) - self.last_instant;
        self.last_instant = cur_secs + cur_nanos_as_secs;
        next_instant
    }
}
