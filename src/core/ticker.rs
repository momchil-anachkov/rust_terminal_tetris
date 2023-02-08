
pub struct Ticker {
    pub tick_interval_time: u128,
    min_tick_interval_time: u128,
    tick_interval_delta: u128,
    time_since_last_tick: u128,
}

impl Ticker {
    pub fn new(tick_interval_time: u128, min_tick_interval_time: u128, tick_interval_delta: u128) -> Ticker {
        return Ticker {
            tick_interval_time,
            min_tick_interval_time,
            tick_interval_delta,
            time_since_last_tick: 0,
        }
    }

    pub fn update(&mut self, delta_time: &u128) -> bool {
        self.time_since_last_tick += delta_time;

        if self.time_since_last_tick > self.tick_interval_time {
            self.time_since_last_tick -= self.tick_interval_time;
            return true;
        }

        return false;
    }

    pub fn increase_tick_speed(&mut self) {
        let new_interval = self.tick_interval_time - self.tick_interval_delta;
        if new_interval < self.min_tick_interval_time {
            self.tick_interval_time = self.min_tick_interval_time;
        } else {
            self.tick_interval_time = new_interval;
        }
    }

    pub fn reset_tick_timer(&mut self) {
        self.time_since_last_tick = 0;
    }
}