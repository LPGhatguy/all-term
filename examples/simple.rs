extern crate all_term;

use all_term::{Style, Color};

use std::{thread, time::Duration};

fn main() {
    let terminal = all_term::terminal();
    let mut handle = terminal.lock().unwrap();

    handle.enable_alternate_screen();
    handle.hide_cursor();

    for i in 0..20 {
        handle.move_cursor(i, i);
        handle.print("ahhh", Style::new().fg(Color::Red).bg(Color::Blue));

        handle.move_cursor(i + 2, i + 3);
        handle.print("ahhh", Style::new().fg(Color::Black).bg(Color::White));

        thread::sleep(Duration::from_millis(40));
        handle.clear_screen();
    }
}