pub mod tetris;

use tetris::TetrisState;

enum PlayingState {
    Running,
    Stopped,
    Paused,
}

struct Game {
    playing_state: PlayingState,
    tetris_state: TetrisState,
    main_menu: Menu,
    pause_menu: Menu,
}

struct Menu {
    items: Vec<String>,
    selected_item: usize,
}