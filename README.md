# Reaction speed test
Extremely simple and fast reaction speed test game.

Shows a full-screen color with some text and waits for a click. Runs at about 4700 FPS on my AMD RX580.

# Usage
Download from [releases](https://github.com/wait-what/reaction-speed/releases) for Windows or Linux (or [build yourself](#building))

- White: click `mouse left` or `space` to start
- Blue: wait for green
- Green: click `mouse left` or `space` as fast as possible
- White again: look at your score and repeat. You will see your last 5 scores and your total average
- Red: you clicked too early

More controls:
- `q` or `esc` to exit
- `r` to reset
- `t` to toggle text

# Building
- Install [Rust](https://www.rust-lang.org/tools/install)
- Clone this repository
- Run `cargo run --release`

# License
[MIT](./LICENSE)
