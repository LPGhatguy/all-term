extern crate all_term;

use all_term::{Style, Color};

use std::{thread, time::Duration};

fn main() {
    let terminal = all_term::terminal();
    let mut handle = terminal.lock().unwrap();

    handle.enable_alternate_screen();
    handle.hide_cursor();

    for i in 0..20 {
        let (width, height) = handle.get_size();
        let text = format!("({}, {})", width, height);

        handle.move_cursor(i, i);
        handle.print(&text, Style::new().fg(Color::Red).bg(Color::Blue));

        handle.move_cursor(i + 2, i + 3);
        handle.print(&text, Style::new().fg(Color::Black).bg(Color::White));

        thread::sleep(Duration::from_millis(200));
        handle.clear_screen();
    }
}