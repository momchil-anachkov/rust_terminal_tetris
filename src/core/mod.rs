pub mod tetris;
pub mod ticker;

use tetris::TetrisState;
use crate::{InputSystem, Tetris};
use crate::core::tetris::MoveOutcome;
use crate::core::ticker::Ticker;

#[derive(PartialEq)]
enum PlayingState {
    Running,
    Stopped,
    Paused,
}

#[derive(PartialEq)]
pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Shift,
    Space,
    P,
    Z,
    X,
    Escape,
    Control,
    C,
    Enter,
}

#[derive(PartialEq)]
pub enum UpdateOutcome {
    Exit,
    Render,
    NothingSpecial,
}

pub struct Menu {
    pub title: &'static str,
    pub items: Vec<&'static MenuItem>,
    pub selected_item: usize,
}

enum Command {
    Pause,
    Resume,
    Stop,
    Start,
    Quit,
}

pub struct MenuItem {
    pub label: &'static str,
    command: Command,
}

impl Menu {
    pub fn new(title: &'static str, items: Vec<&'static MenuItem>) -> Menu {
        return Menu {
            title,
            items,
            selected_item: 0,
        }
    }

    pub fn move_down(&mut self) {
        let len = self.items.len();
        if self.selected_item == len-1 {
            self.selected_item = 0;
        } else {
            self.selected_item += 1;
        }
    }

    pub fn move_up(&mut self) {
        let len = self.items.len();
        if self.selected_item == 0 {
            self.selected_item = len-1;
        } else {
            self.selected_item -= 1;
        }
    }
}

pub struct Game<'a> {
    playing_state: PlayingState,
    tetris: Tetris,
    ticker: &'a mut Ticker,
    pause_menu: Menu,
    main_menu: Menu,
}

pub enum RenderState<'a> {
    Running(TetrisState),
    InMenu(&'a Menu),
}

pub trait Renderer {
    fn render(&mut self, state: &RenderState);
}

impl Game<'_> {
    pub fn new(ticker: &mut Ticker) -> Game {
        return Game {
            playing_state: PlayingState::Stopped,
            tetris: Tetris::new(),
            ticker,
            pause_menu: Menu::new("Paused", Vec::from([
                &MenuItem { label: "Resume",            command: Command::Resume },
                &MenuItem { label: "Exit to Main Menu", command: Command::Stop },
                &MenuItem { label: "Quit",              command: Command::Quit },
            ])),
            main_menu: Menu::new("Welcome to Terminal Tetris!", Vec::from([
                &MenuItem { label: "Start new Game", command: Command::Start },
                &MenuItem { label: "Quit",           command: Command::Quit },
            ])),
        }
    }

    pub fn update(&mut self, keys: &Vec<Key>, delta_time: &u128) -> UpdateOutcome {
        let input_outcome = self.process_input(&keys);

        if input_outcome == UpdateOutcome::Exit {
            return input_outcome;
        }

        if self.playing_state == PlayingState::Running {
            let should_tick = self.ticker.update(&delta_time);

            if should_tick {
                let move_outcome = self.tetris.move_down_and_stick();
                if move_outcome == MoveOutcome::GameOver {
                    return UpdateOutcome::Exit;
                }

                return UpdateOutcome::Render;
            }
        }

        return input_outcome;
    }

    pub fn state(&self) -> RenderState {
        match self.playing_state {
            PlayingState::Running => RenderState::Running(self.tetris.state()),
            PlayingState::Paused => RenderState::InMenu(&self.pause_menu),
            PlayingState::Stopped => RenderState::InMenu(&self.main_menu),
        }
    }

    fn process_input(&mut self, keys: &Vec<Key>) -> UpdateOutcome {
        if keys.contains(&Key::Control) && keys.contains(&Key::C) {
            return UpdateOutcome::Exit;
        }

        let mut update_outcome: UpdateOutcome = UpdateOutcome::NothingSpecial;

        for key in keys {
            match (&self.playing_state, key) {
                (PlayingState::Running, key) => {
                    let move_outcome = match key {
                        Key::Left   => self.tetris.try_and_move_left(),
                        Key::Right  => self.tetris.try_and_move_right(),
                        Key::Down   => self.tetris.try_and_move_down(),
                        Key::Up     => self.tetris.try_and_rotate_clockwise(),
                        Key::Z      => self.tetris.try_and_rotate_clockwise(),
                        Key::X      => self.tetris.try_and_rotate_counterclockwise(),
                        Key::Shift  => self.tetris.hold_piece(),
                        Key::Space  => self.tetris.slam(),
                        Key::P      => { self.playing_state = PlayingState::Paused; MoveOutcome::NothingSpecial },
                        Key::Escape => { self.playing_state = PlayingState::Paused; MoveOutcome::NothingSpecial },
                        _           => MoveOutcome::NothingSpecial,
                    };

                    match move_outcome {
                        MoveOutcome::SpawnedNewPiece     => self.ticker.reset_tick_timer(),
                        MoveOutcome::MadeContactOnBottom => self.ticker.reset_tick_timer(),
                        MoveOutcome::GameOver => { return UpdateOutcome::Exit },
                        _                                => (),
                    }
                }

                (PlayingState::Paused, key) => {
                    match key {
                        Key::Up     =>  self.pause_menu.move_up(),
                        Key::Down   =>  self.pause_menu.move_down(),
                        Key::P      => self.playing_state = PlayingState::Running,
                        Key::Escape => self.playing_state = PlayingState::Running,
                        Key::Enter  => {
                            match self.pause_menu.items[self.pause_menu.selected_item].command {
                                Command::Resume => { self.playing_state = PlayingState::Running }
                                Command::Quit => { return UpdateOutcome::Exit }
                                Command::Stop => { self.playing_state = PlayingState::Stopped }
                                _ => {}
                            }
                        }
                        _ => (),
                    }
                }

                (PlayingState::Stopped, key) => {
                    match key {
                        Key::Up   =>  self.main_menu.move_up(),
                        Key::Down =>  self.main_menu.move_down(),
                        Key::Enter => {
                            match self.main_menu.items[self.main_menu.selected_item].command {
                                Command::Start => {
                                    self.playing_state = PlayingState::Running;
                                    self.tetris = Tetris::new();
                                }
                                Command::Quit => { return UpdateOutcome::Exit }
                                _ => {}
                            }
                        }
                        _ => (),
                    }
                }
            };

            update_outcome = UpdateOutcome::Render;
        }

        return update_outcome;
    }
}