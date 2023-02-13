# Terminal Tetris

Tetris in the terminal. Written in Rust.

##

```shell
git clone https://github.com/momchil-anachkov/rust_terminal_tetris.git
cargo run
```

## Game Feature List:

- ✅ Basic tetris
- ✅ Wall kicks
- ✅ Ghost piece
- ✅ Smart rotations
- ✅ Next pieces sequence
- ✅ Hold piece
- ✅ Tick reset on slam
- ✅ Play/Pause
- ✅ Menu
- ❌ Incrementing speed/levels (classic tetris speed scale)
- ❌ 1 second grace time before stick
- ❌ GIFs for each of the game features
- ❌ Score System

## Code Feature List

- ❌ Tests for the game rules
- ❌ Tests for the input/event system

## Bugs

- ❌ 50% of the time the game picks up the ENTER from running the executable, and selects the first item in the menu 
- ❌ Currently the ticker doesn't use the curve from the levels
- ❌ We still don't freeze the terminal properly, so some input makes it to the terminal after running
- ❌ We don't restore the terminal to the proper state after running (newlines wrapping is busted afterwards)
