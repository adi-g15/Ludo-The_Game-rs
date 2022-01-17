mod display;
mod engine;
mod game;

use display::Display;
use std::{thread, time::Duration};

fn main() {
    // Instead of using Arc on whole struct, only use it on shareable data structures, so there
    // won't be case where a immutable reference is holding the mutex forever
    let mut display = Display::new();
    println!("{:?}", display.get_player_names());
    thread::sleep(Duration::from_millis(3000));
    display.end_display();
}
