extern crate all_term;

use std::{thread, time::Duration};

fn main() {
    let terminal = all_term::terminal();
    let mut handle = terminal.lock().unwrap();

    handle.enable_alternate_screen();

    thread::sleep(Duration::from_secs(1));
}