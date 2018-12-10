extern crate all_term;

use all_term::Key;

fn main() {
    let terminal = all_term::terminal();
    let mut handle = terminal.lock().unwrap();

    handle.enable_raw_mode();

    loop {
        let key = handle.read_key();

        match key {
            Key::Char('q') => break,
            _ => {
                println!("Got key {:?}", key);
            },
        }
    }
}