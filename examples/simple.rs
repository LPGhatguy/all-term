extern crate all_term;

use std::{thread, time::Duration};

fn main() {
    let terminal = all_term::terminal();
    let mut handle = terminal.lock().unwrap();

    handle.enable_alternate_screen();
    handle.hide_cursor();

    for i in 0..20 {
        handle.move_cursor(i, i);
        println!("ahhhh");
        thread::sleep(Duration::from_millis(100));
        handle.clear_screen();
    }
}