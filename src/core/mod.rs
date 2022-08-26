pub mod tetris;
pub mod ticker;

use tetris::TetrisState;
use crate::{InputSystem, Tetris};
use crate::core::PlayingState::{Paused, Running};
use crate::core::tetris::MoveOutcome;
use crate::core::tetris::MoveOutcome::NothingSpecial;
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

struct Menu {
    items: Vec<String>,
    selected_item: usize,
}

pub struct Game<'a> {
    playing_state: PlayingState,
    pub tetris: Tetris,
    input_system: &'a mut InputSystem,
    ticker: &'a mut Ticker,
    renderer: &'a mut dyn Renderer,
}

pub trait Renderer {
    fn render(&mut self, state: &TetrisState);
}

impl Game<'_> {
    pub fn new<'a>(
        renderer: &'a mut dyn Renderer,
        input_system: &'a mut InputSystem,
        ticker: &'a mut Ticker,
    ) -> Game<'a> {
        return Game {
            playing_state: PlayingState::Running,
            tetris: Tetris::new(),
            renderer,
            input_system,
            ticker,
        }
    }

    pub fn update(&mut self, delta_time: &u128) -> bool {
        let keys = self.input_system.get_keys(delta_time);
        let should_close = self.process_input(&keys);

        if self.playing_state == Running {
            let should_tick = self.ticker.update(&delta_time);

            if should_tick {
                self.tetris.move_down_and_stick();
                self.renderer.render(&self.tetris.state());
            }
        }

        return should_close;
    }

    pub fn render(&mut self) {
        self.renderer.render(&self.tetris.state());
    }

    fn process_input(&mut self, keys: &Vec<Key>) -> bool {
        if keys.contains(&Key::Escape) || keys.contains(&Key::Control) && keys.contains(&Key::C) {
            return true;
        }

        for key in keys {
            match (&self.playing_state, key) {
                (PlayingState::Running, key) => {
                    let move_outcome = match key {
                        Key::Left  => self.tetris.try_and_move_left(),
                        Key::Right => self.tetris.try_and_move_right(),
                        Key::Down  => self.tetris.try_and_move_down(),
                        Key::Up    => self.tetris.try_and_rotate_clockwise(),
                        Key::Z     => self.tetris.try_and_rotate_clockwise(),
                        Key::X     => self.tetris.try_and_rotate_counterclockwise(),
                        Key::Shift => self.tetris.hold_piece(),
                        Key::Space => self.tetris.slam(),
                        Key::P     => { self.playing_state = Paused; NothingSpecial },
                        _          => NothingSpecial,
                    };

                    match move_outcome {
                        MoveOutcome::SpawnedNewPiece     => self.ticker.reset_tick_timer(),
                        MoveOutcome::MadeContactOnBottom => self.ticker.reset_tick_timer(),
                        _                                => (),
                    }
                }

                (PlayingState::Paused, key) => {
                    match key {
                        Key::Up   =>  (/* Move up in the pause menu */),
                        Key::Down =>  (/* Move down in the pause menu */),
                        Key::Enter => (/* Select current menu item */),
                        Key::P     => self.playing_state = Running,
                        _ => (),
                    }
                }

                (PlayingState::Stopped, key) => {
                    match key {
                        Key::Up   =>  (/* Move up in the main menu */),
                        Key::Down =>  (/* Move down in the main menu */),
                        Key::Enter => (/* Select current menu item */),
                        _ => (),
                    }
                }

                (_, _) => (),
            };

            self.renderer.render(&self.tetris.state());
        }

        return false;
    }
}