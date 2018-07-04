use std::time::{Instant};
use std::collections::{HashMap};

#[derive(PartialEq, Eq, Hash)]
pub enum TimerType {
    Main,
    Gameplay,
}

pub struct Timers {
    standard_timers: Box<HashMap<TimerType, Timer>>,
    user_timers: Box<HashMap<String, Timer>>,
}

impl Timers {
    pub fn new() -> Timers {
        let mut timers = Timers {
            standard_timers: Box::new(HashMap::new()),
            user_timers: Box::new(HashMap::new())
        };
        timers.create_standard_timers();
        timers
    }

    pub fn update(&mut self) {
        for mut val in self.standard_timers.values_mut() {
            val.update();
        }
    }

    pub fn set_paused(&mut self, is_paused: bool) {
        for mut val in self.standard_timers.values_mut() {
            val.set_paused(is_paused);
        }
    }

    fn create_standard_timers(&mut self) {
        self.standard_timers.insert(TimerType::Main, Timer::new());
        self.standard_timers.insert(TimerType::Gameplay, Timer::new());
    }

    pub fn get_standard_timer(&self, timer_type: TimerType) -> Option<&Timer> {
        self.standard_timers.get(&timer_type)
    }

    pub fn get_user_timer(&self, timer_key: &String) -> Option<&Timer> {
        self.user_timers.get(timer_key)
    }
}

pub struct Timer {
    last_instant: f32,
    delta: f32,
    secs_paused: f32,
    internal_timer: Instant,
    paused_timer: Instant,
    is_paused: bool,
}

impl Timer {
    pub fn new() -> Timer {
        Timer { 
            last_instant: 0.0,
            internal_timer: Instant::now(),
            paused_timer: Instant::now(),
            delta: 0.0,
            secs_paused: 0.0,
            is_paused: false,
        }
    }

    pub fn update(&mut self) {
        if !self.is_paused {
            let time_since_boot = self.get_secs_since_boot();
            self.delta = time_since_boot - self.last_instant;
            self.last_instant = time_since_boot;
        }
    }

    pub fn get_delta(&self) -> f32 {
        self.delta
    }

    pub fn get_secs_since_boot(&self) -> f32 {
        let cur_secs = self.internal_timer.elapsed().as_secs() as f32;
        let cur_nanos_as_secs = (self.internal_timer.elapsed().subsec_nanos() as f32) / 1_000_000_000.0;
        return cur_secs + cur_nanos_as_secs - self.secs_paused
    }

    pub fn set_paused(&mut self, is_paused: bool) {
        self.is_paused = is_paused;
        if self.is_paused {
            self.paused_timer = Instant::now()
        } else {
            let cur_paused_secs = self.paused_timer.elapsed().as_secs() as f32;
            let cur_paused_nanos_as_secs = (self.paused_timer.elapsed().subsec_nanos() as f32) / 1_000_000_000.0;
            self.secs_paused += cur_paused_secs + cur_paused_nanos_as_secs;
        }
    }
}
