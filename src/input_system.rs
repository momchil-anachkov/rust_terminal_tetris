use device_query::{DeviceState, Keycode};
use device_query::DeviceQuery;
use crate::core::Key;

pub struct InputSystem {
    device_state: DeviceState,
    last_frame_keys: Vec<Keycode>,
    current_frame_keys: Vec<Keycode>,
    key_repeat_interval: u128,
    time_since_last_left: u128,
    time_since_last_right: u128,
    time_since_last_down: u128,
}

impl InputSystem {
    pub fn new(key_repeat_interval: u128) -> InputSystem {
        let device_state = DeviceState::new();
        return InputSystem {
            device_state,
            key_repeat_interval,
            last_frame_keys: Vec::new(),
            current_frame_keys: Vec::new(),
            time_since_last_left: 0,
            time_since_last_right: 0,
            time_since_last_down: 0,
        }
    }

    pub fn get_keys(&mut self, delta_time: &u128) -> Vec<Key> {
        let mut keys: Vec<Key> = Vec::new();
        self.current_frame_keys = self.device_state.get_keys();

        if self.is_key_pressed(&Keycode::P) {
            keys.push(Key::P);
        }

        if self.is_key_pressed(&Keycode::Escape) {
            keys.push(Key::Escape);
        }

        if self.current_frame_keys.contains(&Keycode::LControl) {
            keys.push(Key::Control);
        }

        if self.current_frame_keys.contains(&Keycode::C) {
            keys.push(Key::C);
        }

        if self.current_frame_keys.contains(&Keycode::Left) && !self.current_frame_keys.contains(&Keycode::Right) {
            if self.last_frame_keys.contains(&Keycode::Left) {
                self.time_since_last_left += delta_time;
                if self.time_since_last_left > self.key_repeat_interval {
                    self.time_since_last_left -= self.key_repeat_interval;
                    keys.push(Key::Left);
                }
            } else {
                keys.push(Key::Left);
            }
        } else {
            self.time_since_last_left = 0;
        }

        if self.current_frame_keys.contains(&Keycode::Right) && !self.current_frame_keys.contains(&Keycode::Left) {
            if self.last_frame_keys.contains(&Keycode::Right) {
                self.time_since_last_right += delta_time;
                if self.time_since_last_right > self.key_repeat_interval {
                    self.time_since_last_right -= self.key_repeat_interval;
                    keys.push(Key::Right);
                }
            } else {
                keys.push(Key::Right);
            }
        } else {
            self.time_since_last_right = 0;
        }

        if self.current_frame_keys.contains(&Keycode::Down) {
            if self.last_frame_keys.contains(&Keycode::Down) {
                self.time_since_last_down += delta_time;
                if self.time_since_last_down > self.key_repeat_interval {
                    self.time_since_last_down -= self.key_repeat_interval;
                    keys.push(Key::Down);
                }
            } else {
                keys.push(Key::Down);
            }
        } else {
            self.time_since_last_down = 0;
        }

        if self.is_key_pressed(&Keycode::Z) {
            keys.push(Key::Z);
        }

        if self.is_key_pressed(&Keycode::Up) {
            keys.push(Key::Up);
        }

        if self.is_key_pressed(&Keycode::X) {
            keys.push(Key::X);
        }

        if self.is_key_pressed(&Keycode::Space) {
            keys.push(Key::Space);
        }

        if self.is_key_pressed(&Keycode::Enter) {
            keys.push(Key::Enter);
        }

        if self.is_key_pressed(&Keycode::LShift) {
            keys.push(Key::Shift);
        }

        self.last_frame_keys = self.current_frame_keys.clone();

        return keys;
    }

    fn is_key_pressed(&self, key: &Keycode) -> bool {
        return self.current_frame_keys.contains(key) && !self.last_frame_keys.contains(key);
    }
}
