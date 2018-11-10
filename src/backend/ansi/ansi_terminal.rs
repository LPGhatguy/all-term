use std::io::{self, Write};

use crate::{
    terminal_backend::TerminalBackend,
    os::current::{enable_raw_mode, disable_raw_mode},
};

pub struct AnsiTerminal;

const ESC: u8 = 27;

impl TerminalBackend for AnsiTerminal {
    fn enable_raw_mode(&mut self) {
        enable_raw_mode();
    }

    fn disable_raw_mode(&mut self) {
        disable_raw_mode();
    }

    fn enable_alternate_screen(&mut self) {
        // ESC [ ? 1 0 4 9 h
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        handle.write(&[ESC]).unwrap();
        handle.write(b"[?1049h").unwrap();
        handle.flush().unwrap();
    }

    fn disable_alternate_screen(&mut self) {
        // ESC [ ? 1 0 4 9 l
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        handle.write(&[ESC]).unwrap();
        handle.write(b"[?1049l").unwrap();
        handle.flush().unwrap();
    }

    fn clear_screen(&mut self) {
        // ESC [ 2 J
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        handle.write(&[ESC]).unwrap();
        handle.write(b"[2J").unwrap();
        handle.flush().unwrap();
    }

    fn show_cursor(&mut self) {
        // ESC [ ? 25 h
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        handle.write(&[ESC]).unwrap();
        handle.write(b"[?25h").unwrap();
        handle.flush().unwrap();
    }

    fn hide_cursor(&mut self) {
        // ESC [ ? 25 l
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        handle.write(&[ESC]).unwrap();
        handle.write(b"[?25l").unwrap();
        handle.flush().unwrap();
    }
}