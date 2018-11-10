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
    }

    fn disable_alternate_screen(&mut self) {
        // ESC [ ? 1 0 4 9 l
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        handle.write(&[ESC]).unwrap();
        handle.write(b"[?1049l").unwrap();
    }
}