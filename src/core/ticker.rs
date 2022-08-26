
pub struct Ticker {
    tick_interval_time: u128,
    time_since_last_tick: u128,
}

impl Ticker {
    pub(crate) fn new(tick_interval_time: u128) -> Ticker {
        return Ticker {
            tick_interval_time,
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

    pub fn reset_tick_timer(&mut self) {
        self.time_since_last_tick = 0;
    }
}