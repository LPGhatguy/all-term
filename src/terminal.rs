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
}

impl Terminal {
    fn with_backend(backend: Box<TerminalBackend>) -> Terminal {
        Terminal {
            backend,
        }
    }

    pub fn enable_raw_mode(&mut self) {
        self.backend.enable_raw_mode();
    }

    pub fn disable_raw_mode(&mut self) {
        self.backend.disable_raw_mode();
    }
}