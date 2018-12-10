extern crate all_term;

use all_term::{Style};

fn main() {
    let terminal = all_term::terminal();
    let mut handle = terminal.lock().unwrap();

    handle.enable_alternate_screen();
    handle.enable_raw_mode();
    handle.hide_cursor();

    let (width, height) = handle.get_size();

    let horizontal_bar = "=".repeat(width);
    let vertical_bar = "|\n".repeat(height - 2);

    handle.move_cursor(0, 0);
    handle.print(&horizontal_bar, Style::new());

    handle.move_cursor(0, height);
    handle.print(&horizontal_bar, Style::new());

    handle.move_cursor(0, 1);
    handle.print(&vertical_bar, Style::new());

    handle.move_cursor(width, 1);
    handle.print(&vertical_bar, Style::new());

    handle.move_cursor(1, 1);
    handle.print("Press any key to quit", Style::new());

    handle.enable_raw_mode();
    handle.read_key();
}