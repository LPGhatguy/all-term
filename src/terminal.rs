use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use crate::{
    terminal_backend::TerminalBackend,
    backend::ansi::AnsiTerminal,
};

#[cfg(windows)]
fn choose_backend() -> Box<TerminalBackend> {
    use crate::backend::windows::{
        enable_ansi_mode,
        WindowsTerminal,
    };

    if enable_ansi_mode() {
        Box::new(AnsiTerminal)
    } else {
        Box::new(WindowsTerminal)
    }
}

#[cfg(not(windows))]
fn choose_backend() -> Box<TerminalBackend> {
    Box::new(AnsiTerminal)
}

/// Provide's access to the application's terminal.
pub fn terminal() -> Arc<Mutex<Terminal>> {
    lazy_static! {
        static ref TERMINAL: Arc<Mutex<Terminal>> = {
            let backend = choose_backend();
            let terminal = Terminal::with_backend(backend);
            Arc::new(Mutex::new(terminal))
        };
    }

    Arc::clone(&TERMINAL)
}

pub struct Terminal {
    backend: Box<TerminalBackend>,
    is_in_raw_mode: bool,
}

impl Terminal {
    fn with_backend(backend: Box<TerminalBackend>) -> Terminal {
        Terminal {
            backend,
            is_in_raw_mode: false,
        }
    }

    pub fn enable_raw_mode(&mut self) {
        if !self.is_in_raw_mode {
            self.backend.enable_raw_mode();
            self.is_in_raw_mode = true;
        }
    }

    pub fn disable_raw_mode(&mut self) {
        if self.is_in_raw_mode {
            self.backend.disable_raw_mode();
            self.is_in_raw_mode = false;
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.disable_raw_mode();
    }
}