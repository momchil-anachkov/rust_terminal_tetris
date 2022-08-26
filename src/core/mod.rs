pub mod tetris;
pub mod ticker;

use tetris::TetrisState;
use crate::{InputSystem, Tetris};
use crate::core::tetris::MoveOutcome;
use crate::core::tetris::MoveOutcome::{GameOver, NothingSpecial};
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

pub struct Menu <'a> {
    pub items: Vec<&'a str>,
    pub selected_item: usize,
}

impl Menu<'_> {
    pub fn new(items: Vec<&str>) -> Menu {
        return Menu {
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
    pub tetris: Tetris,
    input_system: &'a mut InputSystem,
    ticker: &'a mut Ticker,
    renderer: &'a mut dyn Renderer,
    pause_menu: Menu<'a>,
}

pub enum RenderState<'a> {
    Running(TetrisState),
    Paused(&'a Menu<'a>),
}

pub trait Renderer {
    fn render(&mut self, state: &RenderState);
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
            pause_menu: Menu::new(Vec::from(["Resume", "Exit"])),
        }
    }

    pub fn update(&mut self, delta_time: &u128) -> bool {
        let keys = self.input_system.get_keys(delta_time);
        let should_close = self.process_input(&keys);

        if self.playing_state == PlayingState::Running {
            let should_tick = self.ticker.update(&delta_time);

            if should_tick {
                let move_outcome = self.tetris.move_down_and_stick();
                if move_outcome == GameOver {
                    return true;
                }
                self.render();
            }
        }

        return should_close;
    }

    pub fn render(&mut self) {
        let state = Game::state(&self.playing_state, self.tetris.state(), &self.pause_menu);
        self.renderer.render(&state);
    }

    fn state<'a>(playing_state: &'a PlayingState, tetris_state: TetrisState, pause_menu: &'a Menu) -> RenderState<'a> {
        match playing_state {
            PlayingState::Running => RenderState::Running(tetris_state),
            PlayingState::Paused => RenderState::Paused(pause_menu),
            PlayingState::Stopped => RenderState::Paused(pause_menu),
        }
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
                        Key::P     => { self.playing_state = PlayingState::Paused; NothingSpecial },
                        _          => NothingSpecial,
                    };

                    match move_outcome {
                        MoveOutcome::SpawnedNewPiece     => self.ticker.reset_tick_timer(),
                        MoveOutcome::MadeContactOnBottom => self.ticker.reset_tick_timer(),
                        MoveOutcome::GameOver => { return true },
                        _                                => (),
                    }
                }

                (PlayingState::Paused, key) => {
                    match key {
                        Key::Up   =>  self.pause_menu.move_up(),
                        Key::Down =>  self.pause_menu.move_down(),
                        Key::Enter => (/* Select current menu item */),
                        Key::P     => self.playing_state = PlayingState::Running,
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
            };

            self.renderer.render(&Game::state(&self.playing_state, self.tetris.state(), &self.pause_menu));
        }

        return false;
    }
}