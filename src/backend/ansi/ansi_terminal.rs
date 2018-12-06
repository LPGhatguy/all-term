use std::{
    io::{self, Write},
    sync::mpsc,
    collections::VecDeque,
};

use crate::{
    os::current::{enable_raw_mode, disable_raw_mode, get_terminal_size},
    key::Key,
    style::Style,
    terminal_backend::TerminalBackend,
};

use super::input::{start_input_thread, read_key};

pub struct AnsiTerminal {
    input_receiver: mpsc::Receiver<u8>,
    input_buffer: VecDeque<Key>,
}

impl AnsiTerminal {
    pub fn new() -> AnsiTerminal {
        let (input_sender, input_receiver) = mpsc::channel();

        start_input_thread(input_sender);

        AnsiTerminal {
            input_receiver,
            input_buffer: VecDeque::new(),
        }
    }
}

const ESC: u8 = 27;

impl TerminalBackend for AnsiTerminal {
    fn enable_raw_mode(&mut self)  -> Result<(), String> {
        enable_raw_mode()
    }

    fn disable_raw_mode(&mut self)  -> Result<(), String> {
        disable_raw_mode()
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

    fn move_cursor(&mut self, x: usize, y: usize) {
        // ESC [ <y> ; <x> H
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        handle.write(&[ESC]).unwrap();
        write!(&mut handle, "[{};{}H", y, x).unwrap();
        handle.flush().unwrap();
    }

    #[allow(unused_assignments)]
    fn print(&mut self, text: &str, style: Style) {
        // ESC [ <n> m
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        handle.write(&[ESC, b'[']).unwrap();

        let mut semi = false;
        if let Some(foreground) = style.foreground {
            write!(&mut handle, "{}", foreground.to_ansi_foreground()).unwrap();
            semi = true;
        }

        if let Some(background) = style.background {
            if semi {
                write!(&mut handle, ";").unwrap();
                semi = false;
            }
            write!(&mut handle, "{}", background.to_ansi_background()).unwrap();
        }

        write!(&mut handle, "m").unwrap();
        write!(&mut handle, "{}", text).unwrap();

        handle.write(&[ESC, b'[', b'0', b'm']).unwrap();

        handle.flush().unwrap();
    }

    fn get_size(&self) -> Result<(usize, usize), String> {
        get_terminal_size()
    }

    fn read_key(&mut self) -> Key {
        match self.input_buffer.pop_front() {
            Some(key) => key,
            None => {
                read_key(&self.input_receiver, &mut self.input_buffer);
                self.read_key()
            },
        }
    }
}