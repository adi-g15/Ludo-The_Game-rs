mod display;
use display::Display;
use std::{thread, time::Duration};

fn some_func(display: &mut Display) {
    thread::sleep(Duration::from_millis(2000));
    display.custom_diplay();
}

fn main() {
    // Instead of using Arc on whole struct, only use it on shareable data structures, so there
    // won't be case where a immutable reference is holding the mutex forever
    let mut display = Display::new();
    let display_loop = display.init_display();

    some_func(&mut display);
    thread::sleep(Duration::from_millis(3000));
    display.end_display();
    display_loop.join().unwrap();
}
