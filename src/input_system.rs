use device_query::{DeviceState, Keycode};
use device_query::DeviceQuery;

#[derive(PartialEq)]
pub enum GameMove {
    MoveLeft,
    MoveRight,
    MoveDown,
    RotateClockwise,
    RotateCounterClockwise,
    Tick,
    Slam,
    Hold,
}

#[derive(PartialEq)]
pub enum Command {
    MakeGameMove(GameMove),
    NoOp,
    Exit,
}

pub struct InputSystem {
    device_state: DeviceState,
    is_running: bool,
    last_frame_keys: Vec<Keycode>,
    current_frame_keys: Vec<Keycode>,
    tick_interval_time: u128,
    key_repeat_interval: u128,
    time_since_last_tick: u128,
    time_since_last_left: u128,
    time_since_last_right: u128,
    time_since_last_down: u128,
}

impl InputSystem {
    pub fn new(tick_interval_time: u128, key_repeat_interval: u128) -> InputSystem {
        let device_state = DeviceState::new();
        return InputSystem {
            device_state,
            is_running: false,
            tick_interval_time,
            key_repeat_interval,
            last_frame_keys: Vec::new(),
            current_frame_keys: Vec::new(),
            time_since_last_tick: 0,
            time_since_last_left: 0,
            time_since_last_right: 0,
            time_since_last_down: 0,
        }
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn start(&mut self) {
        self.is_running = true;
    }

    pub fn process_input(&mut self, delta_time: u128) -> Command {
        let keys: Vec<Keycode> = self.device_state.get_keys();
        self.current_frame_keys = keys;

        if self.current_frame_keys.contains(&Keycode::P) {
            if !self.last_frame_keys.contains(&Keycode::P) {
                if self.is_running {
                    self.stop();
                } else {
                    self.start();
                }
            }
        }

        if !self.is_running {
            return self.return_command(Command::NoOp);
        }

        self.time_since_last_tick += delta_time;

        if self.time_since_last_tick > self.tick_interval_time {
            self.time_since_last_tick -= self.tick_interval_time;
            return self.return_command(Command::MakeGameMove(GameMove::Tick));
        }

        if self.current_frame_keys.contains(&Keycode::Left) && self.current_frame_keys.contains(&Keycode::Right) {
            return self.return_command(Command::NoOp);
        }

        if self.current_frame_keys.contains(&Keycode::Escape) || self.current_frame_keys.contains(&Keycode::LControl) && self.current_frame_keys.contains(&Keycode::C) {
            return self.return_command(Command::Exit);
        }

        if self.current_frame_keys.contains(&Keycode::Left) {
            if self.last_frame_keys.contains(&Keycode::Left) {
                self.time_since_last_left += delta_time;
                if self.time_since_last_left > self.key_repeat_interval {
                    self.time_since_last_left -= self.key_repeat_interval;
                    return self.return_command(Command::MakeGameMove(GameMove::MoveLeft));
                }
            } else {
                return self.return_command(Command::MakeGameMove(GameMove::MoveLeft));
            }
        } else {
            self.time_since_last_left = 0;
        }

        if self.current_frame_keys.contains(&Keycode::Right) {
            if self.last_frame_keys.contains(&Keycode::Right) {
                self.time_since_last_right += delta_time;
                if self.time_since_last_right > self.key_repeat_interval {
                    self.time_since_last_right -= self.key_repeat_interval;
                    return self.return_command(Command::MakeGameMove(GameMove::MoveRight));
                }
            } else {
                return self.return_command(Command::MakeGameMove(GameMove::MoveRight));
            }
        } else {
            self.time_since_last_right = 0;
        }

        if self.current_frame_keys.contains(&Keycode::Down) {
            if self.last_frame_keys.contains(&Keycode::Down) {
                self.time_since_last_down += delta_time;
                if self.time_since_last_down > self.key_repeat_interval {
                    self.time_since_last_down -= self.key_repeat_interval;
                    return self.return_command(Command::MakeGameMove(GameMove::MoveDown));
                }
            } else {
                return self.return_command(Command::MakeGameMove(GameMove::MoveDown));
            }
        } else {
            self.time_since_last_down = 0;
        }

        if self.current_frame_keys.contains(&Keycode::Z) || self.current_frame_keys.contains(&Keycode::Up) {
            if !self.last_frame_keys.contains(&Keycode::Z) && !self.last_frame_keys.contains(&Keycode::Up) {
                return self.return_command(Command::MakeGameMove(GameMove::RotateClockwise));
            }
        }

        if self.current_frame_keys.contains(&Keycode::X) {
            if !self.last_frame_keys.contains(&Keycode::X) {
                return self.return_command(Command::MakeGameMove(GameMove::RotateCounterClockwise));
            }
        }

        if self.current_frame_keys.contains(&Keycode::Space) {
            if !self.last_frame_keys.contains(&Keycode::Space) {
                return self.return_command(Command::MakeGameMove(GameMove::Slam));
            }
        }

        if self.current_frame_keys.contains(&Keycode::LShift) {
            if !self.last_frame_keys.contains(&Keycode::LShift) {
                return self.return_command(Command::MakeGameMove(GameMove::Hold));
            }
        }

        return self.return_command(Command::NoOp);
    }

    pub fn reset_tick_timer(&mut self) {
        self.time_since_last_tick = 0;
    }

    fn return_command(&mut self, command: Command) -> Command {
        self.last_frame_keys = self.current_frame_keys.clone();
        return command;
    }
}
