extern crate all_term;

use all_term::{Key, Style};

fn main() {
    let terminal = all_term::terminal();
    let mut handle = terminal.lock().unwrap();

    handle.enable_raw_mode();
    handle.enable_alternate_screen();
    handle.hide_cursor();

    loop {
        let key = handle.read_key();

        match key {
            Key::Char('q') => break,
            _ => {
                handle.clear_screen();
                handle.move_cursor(0, 0);
                handle.print(format!("Got key {:?}", key), Style::new());
            },
        }
    }
}