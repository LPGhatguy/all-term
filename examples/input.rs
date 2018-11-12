extern crate all_term;

use all_term::{Style, Color, Key};

use std::{thread, time::Duration};

fn main() {
    let terminal = all_term::terminal();
    let mut handle = terminal.lock().unwrap();

    handle.enable_raw_mode();
    handle.enable_alternate_screen();

    loop {
        let key = handle.read_key();
        handle.clear_screen();

        match key {
            Key::Char('q') => break,
            _ => println!("Got key! {:?}", key),
        }
    }
}