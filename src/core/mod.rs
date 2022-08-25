pub mod tetris;

use tetris::TetrisState;
use crate::{InputSystem, Tetris};

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
    Control,
    Shift,
    Space,
    Escape,
    C,
    P,
    Z,
    X,
}

struct Menu {
    items: Vec<String>,
    selected_item: usize,
}

pub struct Game<'a> {
    playing_state: PlayingState,
    pub tetris: Tetris,
    input_system: &'a mut InputSystem,
    renderer: &'a mut dyn Renderer,
    // main_menu: Menu,
    // pause_menu: Menu,
}

pub trait Renderer {
    fn render(&mut self, state: &TetrisState);
}

impl Game<'_> {
    pub fn new<'a>(renderer: &'a mut dyn Renderer, input_system: &'a mut InputSystem) -> Game<'a> {
        return Game {
            playing_state: PlayingState::Running,
            tetris: Tetris::new(),
            renderer,
            input_system
        }
    }

    pub fn update(&mut self, delta_time: &u128) -> bool {
        let keys = self.input_system.get_keys(delta_time);
        return self.process_input(&keys);
    }

    fn process_input(&mut self, keys: &Vec<Key>) -> bool {
        if keys.contains(&Key::Escape) || keys.contains(&Key::Control) && keys.contains(&Key::C) {
            return true;
        }

        for key in keys {
            match (&self.playing_state, key) {
                (_, Key::Left) => self.tetris.try_and_move_left(),
                (_, Key::Right) => self.tetris.try_and_move_right(),
                (_, Key::Down) => self.tetris.try_and_move_down(),
                (_, Key::Up | Key::Z) => self.tetris.rotate_clockwise(),
                (_, Key::X) => self.tetris.rotate_counterclockwise(),
                (_, Key::Shift) => self.tetris.hold_piece(),
                (_, Key::Space) => { self.tetris.slam(); },
                (_, _) => (),
            }

            self.renderer.render(&self.tetris.state());
        }

        return false;
    }
}